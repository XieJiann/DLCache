# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: dataloader.proto
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()


import common_pb2 as common__pb2


DESCRIPTOR = _descriptor.FileDescriptor(
  name='dataloader.proto',
  package='dataloader',
  syntax='proto3',
  serialized_options=None,
  create_key=_descriptor._internal_create_key,
  serialized_pb=b'\n\x10\x64\x61taloader.proto\x12\ndataloader\x1a\x0c\x63ommon.proto\"\'\n\x17\x43reateDataloaderRequest\x12\x0c\n\x04name\x18\x01 \x01(\t\"_\n\x18\x43reateDataloaderResponse\x12\x10\n\x08shm_path\x18\x02 \x01(\t\x12\x11\n\tloader_id\x18\x03 \x01(\x04\x12\x1e\n\x06status\x18\x04 \x01(\x0b\x32\x0e.common.Status\" \n\x0bNextRequest\x12\x11\n\tloader_id\x18\x01 \x01(\x04\"\x1f\n\x0cNextResponse\x12\x0f\n\x07\x61\x64\x64ress\x18\x02 \x03(\x04\",\n\x17\x44\x65leteDataloaderRequest\x12\x11\n\tloader_id\x18\x03 \x01(\x04\":\n\x18\x44\x65leteDataloaderResponse\x12\x1e\n\x06status\x18\x03 \x01(\x0b\x32\x0e.common.Status2\x88\x02\n\rDataLoaderSvc\x12]\n\x10\x43reateDataloader\x12#.dataloader.CreateDataloaderRequest\x1a$.dataloader.CreateDataloaderResponse\x12\x39\n\x04Next\x12\x17.dataloader.NextRequest\x1a\x18.dataloader.NextResponse\x12]\n\x10\x44\x65leteDataloader\x12#.dataloader.DeleteDataloaderRequest\x1a$.dataloader.DeleteDataloaderResponseb\x06proto3'
  ,
  dependencies=[common__pb2.DESCRIPTOR,])




_CREATEDATALOADERREQUEST = _descriptor.Descriptor(
  name='CreateDataloaderRequest',
  full_name='dataloader.CreateDataloaderRequest',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  create_key=_descriptor._internal_create_key,
  fields=[
    _descriptor.FieldDescriptor(
      name='name', full_name='dataloader.CreateDataloaderRequest.name', index=0,
      number=1, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=b"".decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=46,
  serialized_end=85,
)


_CREATEDATALOADERRESPONSE = _descriptor.Descriptor(
  name='CreateDataloaderResponse',
  full_name='dataloader.CreateDataloaderResponse',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  create_key=_descriptor._internal_create_key,
  fields=[
    _descriptor.FieldDescriptor(
      name='shm_path', full_name='dataloader.CreateDataloaderResponse.shm_path', index=0,
      number=2, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=b"".decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='loader_id', full_name='dataloader.CreateDataloaderResponse.loader_id', index=1,
      number=3, type=4, cpp_type=4, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='status', full_name='dataloader.CreateDataloaderResponse.status', index=2,
      number=4, type=11, cpp_type=10, label=1,
      has_default_value=False, default_value=None,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=87,
  serialized_end=182,
)


_NEXTREQUEST = _descriptor.Descriptor(
  name='NextRequest',
  full_name='dataloader.NextRequest',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  create_key=_descriptor._internal_create_key,
  fields=[
    _descriptor.FieldDescriptor(
      name='loader_id', full_name='dataloader.NextRequest.loader_id', index=0,
      number=1, type=4, cpp_type=4, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=184,
  serialized_end=216,
)


_NEXTRESPONSE = _descriptor.Descriptor(
  name='NextResponse',
  full_name='dataloader.NextResponse',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  create_key=_descriptor._internal_create_key,
  fields=[
    _descriptor.FieldDescriptor(
      name='address', full_name='dataloader.NextResponse.address', index=0,
      number=2, type=4, cpp_type=4, label=3,
      has_default_value=False, default_value=[],
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=218,
  serialized_end=249,
)


_DELETEDATALOADERREQUEST = _descriptor.Descriptor(
  name='DeleteDataloaderRequest',
  full_name='dataloader.DeleteDataloaderRequest',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  create_key=_descriptor._internal_create_key,
  fields=[
    _descriptor.FieldDescriptor(
      name='loader_id', full_name='dataloader.DeleteDataloaderRequest.loader_id', index=0,
      number=3, type=4, cpp_type=4, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=251,
  serialized_end=295,
)


_DELETEDATALOADERRESPONSE = _descriptor.Descriptor(
  name='DeleteDataloaderResponse',
  full_name='dataloader.DeleteDataloaderResponse',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  create_key=_descriptor._internal_create_key,
  fields=[
    _descriptor.FieldDescriptor(
      name='status', full_name='dataloader.DeleteDataloaderResponse.status', index=0,
      number=3, type=11, cpp_type=10, label=1,
      has_default_value=False, default_value=None,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=297,
  serialized_end=355,
)

_CREATEDATALOADERRESPONSE.fields_by_name['status'].message_type = common__pb2._STATUS
_DELETEDATALOADERRESPONSE.fields_by_name['status'].message_type = common__pb2._STATUS
DESCRIPTOR.message_types_by_name['CreateDataloaderRequest'] = _CREATEDATALOADERREQUEST
DESCRIPTOR.message_types_by_name['CreateDataloaderResponse'] = _CREATEDATALOADERRESPONSE
DESCRIPTOR.message_types_by_name['NextRequest'] = _NEXTREQUEST
DESCRIPTOR.message_types_by_name['NextResponse'] = _NEXTRESPONSE
DESCRIPTOR.message_types_by_name['DeleteDataloaderRequest'] = _DELETEDATALOADERREQUEST
DESCRIPTOR.message_types_by_name['DeleteDataloaderResponse'] = _DELETEDATALOADERRESPONSE
_sym_db.RegisterFileDescriptor(DESCRIPTOR)

CreateDataloaderRequest = _reflection.GeneratedProtocolMessageType('CreateDataloaderRequest', (_message.Message,), {
  'DESCRIPTOR' : _CREATEDATALOADERREQUEST,
  '__module__' : 'dataloader_pb2'
  # @@protoc_insertion_point(class_scope:dataloader.CreateDataloaderRequest)
  })
_sym_db.RegisterMessage(CreateDataloaderRequest)

CreateDataloaderResponse = _reflection.GeneratedProtocolMessageType('CreateDataloaderResponse', (_message.Message,), {
  'DESCRIPTOR' : _CREATEDATALOADERRESPONSE,
  '__module__' : 'dataloader_pb2'
  # @@protoc_insertion_point(class_scope:dataloader.CreateDataloaderResponse)
  })
_sym_db.RegisterMessage(CreateDataloaderResponse)

NextRequest = _reflection.GeneratedProtocolMessageType('NextRequest', (_message.Message,), {
  'DESCRIPTOR' : _NEXTREQUEST,
  '__module__' : 'dataloader_pb2'
  # @@protoc_insertion_point(class_scope:dataloader.NextRequest)
  })
_sym_db.RegisterMessage(NextRequest)

NextResponse = _reflection.GeneratedProtocolMessageType('NextResponse', (_message.Message,), {
  'DESCRIPTOR' : _NEXTRESPONSE,
  '__module__' : 'dataloader_pb2'
  # @@protoc_insertion_point(class_scope:dataloader.NextResponse)
  })
_sym_db.RegisterMessage(NextResponse)

DeleteDataloaderRequest = _reflection.GeneratedProtocolMessageType('DeleteDataloaderRequest', (_message.Message,), {
  'DESCRIPTOR' : _DELETEDATALOADERREQUEST,
  '__module__' : 'dataloader_pb2'
  # @@protoc_insertion_point(class_scope:dataloader.DeleteDataloaderRequest)
  })
_sym_db.RegisterMessage(DeleteDataloaderRequest)

DeleteDataloaderResponse = _reflection.GeneratedProtocolMessageType('DeleteDataloaderResponse', (_message.Message,), {
  'DESCRIPTOR' : _DELETEDATALOADERRESPONSE,
  '__module__' : 'dataloader_pb2'
  # @@protoc_insertion_point(class_scope:dataloader.DeleteDataloaderResponse)
  })
_sym_db.RegisterMessage(DeleteDataloaderResponse)



_DATALOADERSVC = _descriptor.ServiceDescriptor(
  name='DataLoaderSvc',
  full_name='dataloader.DataLoaderSvc',
  file=DESCRIPTOR,
  index=0,
  serialized_options=None,
  create_key=_descriptor._internal_create_key,
  serialized_start=358,
  serialized_end=622,
  methods=[
  _descriptor.MethodDescriptor(
    name='CreateDataloader',
    full_name='dataloader.DataLoaderSvc.CreateDataloader',
    index=0,
    containing_service=None,
    input_type=_CREATEDATALOADERREQUEST,
    output_type=_CREATEDATALOADERRESPONSE,
    serialized_options=None,
    create_key=_descriptor._internal_create_key,
  ),
  _descriptor.MethodDescriptor(
    name='Next',
    full_name='dataloader.DataLoaderSvc.Next',
    index=1,
    containing_service=None,
    input_type=_NEXTREQUEST,
    output_type=_NEXTRESPONSE,
    serialized_options=None,
    create_key=_descriptor._internal_create_key,
  ),
  _descriptor.MethodDescriptor(
    name='DeleteDataloader',
    full_name='dataloader.DataLoaderSvc.DeleteDataloader',
    index=2,
    containing_service=None,
    input_type=_DELETEDATALOADERREQUEST,
    output_type=_DELETEDATALOADERRESPONSE,
    serialized_options=None,
    create_key=_descriptor._internal_create_key,
  ),
])
_sym_db.RegisterServiceDescriptor(_DATALOADERSVC)

DESCRIPTOR.services_by_name['DataLoaderSvc'] = _DATALOADERSVC

# @@protoc_insertion_point(module_scope)
