// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'all.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

AuthBody _$AuthBodyFromJson(Map<String, dynamic> json) => AuthBody(
  accessToken: json['accessToken'] as String,
  refreshToken: json['refreshToken'] as String,
  tokenType: json['tokenType'] as String,
);

Map<String, dynamic> _$AuthBodyToJson(AuthBody instance) => <String, dynamic>{
  'accessToken': instance.accessToken,
  'refreshToken': instance.refreshToken,
  'tokenType': instance.tokenType,
};

AuthInput _$AuthInputFromJson(Map<String, dynamic> json) => AuthInput(
  name: json['name'] as String?,
  email: json['email'] as String,
  password: json['password'] as String,
);

Map<String, dynamic> _$AuthInputToJson(AuthInput instance) => <String, dynamic>{
  'name': instance.name,
  'email': instance.email,
  'password': instance.password,
};

Data<T> _$DataFromJson<T>(
  Map<String, dynamic> json,
  T Function(Object? json) fromJsonT,
) => Data<T>(
  filter:
      json['filter'] == null
          ? null
          : FilterType.fromJson(json['filter'] as Map<String, dynamic>),
  sort:
      json['sort'] == null
          ? null
          : SortType.fromJson(json['sort'] as Map<String, dynamic>),
  relation:
      (json['relation'] as List<dynamic>?)
          ?.map((e) => Relation.fromJson(e as Map<String, dynamic>))
          .toList(),
  data: _$nullableGenericFromJson(json['data'], fromJsonT),
);

Map<String, dynamic> _$DataToJson<T>(
  Data<T> instance,
  Object? Function(T value) toJsonT,
) => <String, dynamic>{
  'filter': instance.filter,
  'sort': instance.sort,
  'relation': instance.relation,
  'data': _$nullableGenericToJson(instance.data, toJsonT),
};

T? _$nullableGenericFromJson<T>(
  Object? input,
  T Function(Object? json) fromJson,
) => input == null ? null : fromJson(input);

Object? _$nullableGenericToJson<T>(
  T? input,
  Object? Function(T value) toJson,
) => input == null ? null : toJson(input);

Relation _$RelationFromJson(Map<String, dynamic> json) => Relation(
  table: json['table'] as String,
  key: json['key'] as String,
  relationName: json['relationName'] as String?,
);

Map<String, dynamic> _$RelationToJson(Relation instance) => <String, dynamic>{
  'table': instance.table,
  'key': instance.key,
  'relationName': instance.relationName,
};

SocketResponse _$SocketResponseFromJson(Map<String, dynamic> json) =>
    SocketResponse(method: json['method'] as String, data: json['data']);

Map<String, dynamic> _$SocketResponseToJson(SocketResponse instance) =>
    <String, dynamic>{'method': instance.method, 'data': instance.data};

WebSocketResponse _$WebSocketResponseFromJson(Map<String, dynamic> json) =>
    WebSocketResponse(
      from: json['from'] as String,
      method: json['method'] as String?,
      path: json['path'] as String?,
      data: json['data'],
    );

Map<String, dynamic> _$WebSocketResponseToJson(WebSocketResponse instance) =>
    <String, dynamic>{
      'from': instance.from,
      'method': instance.method,
      'path': instance.path,
      'data': instance.data,
    };

WebSocketRequest _$WebSocketRequestFromJson(Map<String, dynamic> json) =>
    WebSocketRequest(type: json['type'] as String, data: json['data']);

Map<String, dynamic> _$WebSocketRequestToJson(WebSocketRequest instance) =>
    <String, dynamic>{'type': instance.type, 'data': instance.data};

FilterType _$FilterTypeFromJson(Map<String, dynamic> json) => FilterType(
  type: json['type'] as String,
  key: json['key'],
  value: json['value'],
  pattern: json['pattern'],
  left:
      json['left'] == null
          ? null
          : FilterType.fromJson(json['left'] as Map<String, dynamic>),
  right:
      json['right'] == null
          ? null
          : FilterType.fromJson(json['right'] as Map<String, dynamic>),
  inner:
      json['inner'] == null
          ? null
          : FilterType.fromJson(json['inner'] as Map<String, dynamic>),
);

Map<String, dynamic> _$FilterTypeToJson(FilterType instance) =>
    <String, dynamic>{
      'type': instance.type,
      'key': instance.key,
      'value': instance.value,
      'pattern': instance.pattern,
      'left': instance.left,
      'right': instance.right,
      'inner': instance.inner,
    };

SortType _$SortTypeFromJson(Map<String, dynamic> json) =>
    SortType(type: json['type'] as String, key: json['key']);

Map<String, dynamic> _$SortTypeToJson(SortType instance) => <String, dynamic>{
  'type': instance.type,
  'key': instance.key,
};

Post _$PostFromJson(Map<String, dynamic> json) => Post(
  id: (json['id'] as num?)?.toInt(),
  title: json['title'] as String,
  content: json['content'] as String,
);

Map<String, dynamic> _$PostToJson(Post instance) => <String, dynamic>{
  'id': instance.id,
  'title': instance.title,
  'content': instance.content,
};

Todo _$TodoFromJson(Map<String, dynamic> json) => Todo(
  id: (json['id'] as num).toInt(),
  task: json['task'] as String,
  isCompleted: json['isCompleted'] as bool,
);

Map<String, dynamic> _$TodoToJson(Todo instance) => <String, dynamic>{
  'id': instance.id,
  'task': instance.task,
  'isCompleted': instance.isCompleted,
};

Thread _$ThreadFromJson(Map<String, dynamic> json) => Thread(
  id: (json['id'] as num).toInt(),
  title: json['title'] as String,
  author: json['author'] as String,
  contentId: json['contentId'] as String,
  createdAt: DateTime.parse(json['createdAt'] as String),
  content:
      json['content'] == null
          ? null
          : Content.fromJson(json['content'] as Map<String, dynamic>),
);

Map<String, dynamic> _$ThreadToJson(Thread instance) => <String, dynamic>{
  'id': instance.id,
  'title': instance.title,
  'author': instance.author,
  'contentId': instance.contentId,
  'createdAt': instance.createdAt.toIso8601String(),
  'content': instance.content,
};

Content _$ContentFromJson(Map<String, dynamic> json) => Content();

Map<String, dynamic> _$ContentToJson(Content instance) => <String, dynamic>{};

TextContent _$TextContentFromJson(Map<String, dynamic> json) =>
    TextContent(text: json['text'] as String);

Map<String, dynamic> _$TextContentToJson(TextContent instance) =>
    <String, dynamic>{'text': instance.text};

ImageContent _$ImageContentFromJson(Map<String, dynamic> json) =>
    ImageContent(imageUrl: json['imageUrl'] as String);

Map<String, dynamic> _$ImageContentToJson(ImageContent instance) =>
    <String, dynamic>{'imageUrl': instance.imageUrl};

LinkContent _$LinkContentFromJson(Map<String, dynamic> json) => LinkContent(
  url: json['url'] as String,
  preview: json['preview'] as String?,
);

Map<String, dynamic> _$LinkContentToJson(LinkContent instance) =>
    <String, dynamic>{'url': instance.url, 'preview': instance.preview};
