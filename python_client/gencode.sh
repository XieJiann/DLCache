python3 -m grpc_tools.protoc -I../protos --python_out=. --grpc_python_out=. ../protos/dataloader.proto
python3 -m grpc_tools.protoc -I../protos --python_out=. --grpc_python_out=. ../protos/dataset.proto
python3 -m grpc_tools.protoc -I../protos --python_out=. --grpc_python_out=. ../protos/common.proto