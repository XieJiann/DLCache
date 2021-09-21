# Generated by the gRPC Python protocol compiler plugin. DO NOT EDIT!
"""Client and server classes corresponding to protobuf-defined services."""
import grpc

import dataset_pb2 as dataset__pb2


class DatasetStub(object):
    """Missing associated documentation comment in .proto file."""

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.CreateDataset = channel.unary_unary(
                '/Dataset/CreateDataset',
                request_serializer=dataset__pb2.CreateDatasetRequest.SerializeToString,
                response_deserializer=dataset__pb2.CreateDatasetResponse.FromString,
                )
        self.DeleteDataset = channel.unary_unary(
                '/Dataset/DeleteDataset',
                request_serializer=dataset__pb2.DeleteDatasetRequest.SerializeToString,
                response_deserializer=dataset__pb2.DeleteDatasetResponse.FromString,
                )


class DatasetServicer(object):
    """Missing associated documentation comment in .proto file."""

    def CreateDataset(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def DeleteDataset(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')


def add_DatasetServicer_to_server(servicer, server):
    rpc_method_handlers = {
            'CreateDataset': grpc.unary_unary_rpc_method_handler(
                    servicer.CreateDataset,
                    request_deserializer=dataset__pb2.CreateDatasetRequest.FromString,
                    response_serializer=dataset__pb2.CreateDatasetResponse.SerializeToString,
            ),
            'DeleteDataset': grpc.unary_unary_rpc_method_handler(
                    servicer.DeleteDataset,
                    request_deserializer=dataset__pb2.DeleteDatasetRequest.FromString,
                    response_serializer=dataset__pb2.DeleteDatasetResponse.SerializeToString,
            ),
    }
    generic_handler = grpc.method_handlers_generic_handler(
            'Dataset', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))


 # This class is part of an EXPERIMENTAL API.
class Dataset(object):
    """Missing associated documentation comment in .proto file."""

    @staticmethod
    def CreateDataset(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(request, target, '/Dataset/CreateDataset',
            dataset__pb2.CreateDatasetRequest.SerializeToString,
            dataset__pb2.CreateDatasetResponse.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def DeleteDataset(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(request, target, '/Dataset/DeleteDataset',
            dataset__pb2.DeleteDatasetRequest.SerializeToString,
            dataset__pb2.DeleteDatasetResponse.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)
