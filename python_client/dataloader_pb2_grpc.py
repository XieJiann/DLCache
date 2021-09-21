# Generated by the gRPC Python protocol compiler plugin. DO NOT EDIT!
"""Client and server classes corresponding to protobuf-defined services."""
import grpc

import dataloader_pb2 as dataloader__pb2


class DataLoaderStub(object):
    """Missing associated documentation comment in .proto file."""

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.CreateDataloader = channel.unary_unary(
                '/DataLoader/CreateDataloader',
                request_serializer=dataloader__pb2.CreateDataloaderRequest.SerializeToString,
                response_deserializer=dataloader__pb2.CreateDataloaderResponse.FromString,
                )
        self.Next = channel.unary_unary(
                '/DataLoader/Next',
                request_serializer=dataloader__pb2.NextRequest.SerializeToString,
                response_deserializer=dataloader__pb2.NextResponse.FromString,
                )
        self.DeleteDataloader = channel.unary_unary(
                '/DataLoader/DeleteDataloader',
                request_serializer=dataloader__pb2.DeleteDataloaderRequest.SerializeToString,
                response_deserializer=dataloader__pb2.DeleteDataloaderResponse.FromString,
                )


class DataLoaderServicer(object):
    """Missing associated documentation comment in .proto file."""

    def CreateDataloader(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def Next(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def DeleteDataloader(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')


def add_DataLoaderServicer_to_server(servicer, server):
    rpc_method_handlers = {
            'CreateDataloader': grpc.unary_unary_rpc_method_handler(
                    servicer.CreateDataloader,
                    request_deserializer=dataloader__pb2.CreateDataloaderRequest.FromString,
                    response_serializer=dataloader__pb2.CreateDataloaderResponse.SerializeToString,
            ),
            'Next': grpc.unary_unary_rpc_method_handler(
                    servicer.Next,
                    request_deserializer=dataloader__pb2.NextRequest.FromString,
                    response_serializer=dataloader__pb2.NextResponse.SerializeToString,
            ),
            'DeleteDataloader': grpc.unary_unary_rpc_method_handler(
                    servicer.DeleteDataloader,
                    request_deserializer=dataloader__pb2.DeleteDataloaderRequest.FromString,
                    response_serializer=dataloader__pb2.DeleteDataloaderResponse.SerializeToString,
            ),
    }
    generic_handler = grpc.method_handlers_generic_handler(
            'DataLoader', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))


 # This class is part of an EXPERIMENTAL API.
class DataLoader(object):
    """Missing associated documentation comment in .proto file."""

    @staticmethod
    def CreateDataloader(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(request, target, '/DataLoader/CreateDataloader',
            dataloader__pb2.CreateDataloaderRequest.SerializeToString,
            dataloader__pb2.CreateDataloaderResponse.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def Next(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(request, target, '/DataLoader/Next',
            dataloader__pb2.NextRequest.SerializeToString,
            dataloader__pb2.NextResponse.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def DeleteDataloader(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(request, target, '/DataLoader/DeleteDataloader',
            dataloader__pb2.DeleteDataloaderRequest.SerializeToString,
            dataloader__pb2.DeleteDataloaderResponse.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)