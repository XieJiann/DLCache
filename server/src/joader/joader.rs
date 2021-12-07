use crate::cache::cache::Cache;
use crate::dataset::DatasetRef;
use crate::loader::Loader;
use crate::sampler::sampler_tree::SamplerTree;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Joader {
    dataset: DatasetRef,
    sampler_tree: SamplerTree,
    // map loader id to loader
    loader_table: HashMap<u64, Loader>,
    ref_table: HashMap<u32, usize>,
    key: u32,
}

impl Joader {
    fn get_ref_cnt(&mut self, idx: u32, count: usize) -> usize {
        *self.ref_table.get_mut(&idx).unwrap() -= count;
        self.ref_table[&idx]
    }

    pub fn contains(&self, id: u64) -> bool {
        self.loader_table.contains_key(&id)
    }

    pub fn set_hash_key(&mut self, num: u32) {
        self.key = num + 1;
    }

    pub fn get_mut(&mut self, id: u64) -> Result<&mut Loader, String> {
        self.loader_table
            .get_mut(&id)
            .ok_or_else(|| format!("Loader {} does not existed!", id))
    }

    pub fn new(dataset: DatasetRef) -> Joader {
        let mut ref_table = HashMap::new();
        for i in dataset.get_indices() {
            ref_table.insert(i, 0);
        }
        Joader {
            dataset,
            sampler_tree: SamplerTree::new(),
            loader_table: HashMap::new(),
            ref_table,
            key: 0,
        }
    }

    #[inline]
    fn get_hash_host(&self, idx: u32) -> u32 {
        idx % self.key
    }

    pub async fn next(&mut self, cache: &mut Cache) {
        let mut data_table = self.sampler_tree.sample();
        for (data_idx, loader_ids) in data_table.iter_mut() {
            let ref_cnt = self.get_ref_cnt(*data_idx, loader_ids.len());
            let addr = self.dataset.read(cache, *data_idx, ref_cnt);
            let host_id = self.get_hash_host(*data_idx);
            if host_id != self.key - 1 {
                let loader_id_cloned = loader_ids.clone();
                for loader_id in loader_id_cloned {
                    if self
                        .loader_table
                        .get_mut(&loader_id)
                        .unwrap()
                        .send_idx(*data_idx, host_id as u64)
                        .await
                    {
                        // we need distributed the idx to other hosts
                        loader_ids.remove(&loader_id);
                    }
                }
            }

            for id in loader_ids.iter() {
                log::debug!("Joader send data {:} to {:?}", addr, self.loader_table[id]);
                self.loader_table[id].send_data(addr).await;
            }
        }
    }

    pub fn del(&mut self, id: u64) -> Result<(), String> {
        let valuse = self.sampler_tree.get_loader_values(id);
        self.sampler_tree.delete(id);
        for v in valuse.iter() {
            *self.ref_table.get_mut(v).unwrap() -= 1;
        }
        self.loader_table.remove(&id);
        Ok(())
    }

    pub fn add_loader(&mut self, loader_id: u64) {
        self.loader_table.insert(loader_id, Loader::new(loader_id));
    }

    pub fn get_mut_loader(&mut self, id: u64) -> &mut Loader {
        self.loader_table.get_mut(&id).unwrap()
    }

    pub fn get_name(&self) -> &str {
        self.dataset.get_name()
    }

    pub fn is_empty(&self) -> bool {
        self.sampler_tree.is_empty()
    }

    pub fn len(&self) -> u64 {
        self.dataset.len()
    }
}
