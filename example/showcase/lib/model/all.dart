import 'package:json_annotation/json_annotation.dart';

part 'all.g.dart';

@JsonSerializable()
class AuthBody {
  final String accessToken;
  final String refreshToken;
  final String tokenType;

  AuthBody({
    required this.accessToken,
    required this.refreshToken,
    required this.tokenType,
  });

  factory AuthBody.fromJson(Map<String, dynamic> json) =>
      _$AuthBodyFromJson(json);
  Map<String, dynamic> toJson() => _$AuthBodyToJson(this);
}

@JsonSerializable()
class AuthInput {
  final String? name;
  final String email;
  final String password;

  AuthInput({this.name, required this.email, required this.password});

  factory AuthInput.fromJson(Map<String, dynamic> json) =>
      _$AuthInputFromJson(json);
  Map<String, dynamic> toJson() => _$AuthInputToJson(this);
}

@JsonSerializable(genericArgumentFactories: true)
class Data<T> {
  final FilterType? filter;
  final SortType? sort;
  final List<Relation>? relation;
  final T? data;

  Data({this.filter, this.sort, this.relation, this.data});

  factory Data.fromJson(
    Map<String, dynamic> json,
    T Function(Object? json) fromJsonT,
  ) => _$DataFromJson(json, fromJsonT);

  Map<String, dynamic> toJson(Object? Function(T value) toJsonT) =>
      _$DataToJson(this, toJsonT);
}

@JsonSerializable()
class Relation {
  final String table;
  final String key;
  final String? relationName;

  Relation({required this.table, required this.key, this.relationName});

  factory Relation.fromJson(Map<String, dynamic> json) =>
      _$RelationFromJson(json);
  Map<String, dynamic> toJson() => _$RelationToJson(this);
}

@JsonSerializable()
class SocketResponse {
  final String method;
  final dynamic data;

  SocketResponse({required this.method, required this.data});

  factory SocketResponse.fromJson(Map<String, dynamic> json) =>
      _$SocketResponseFromJson(json);
  Map<String, dynamic> toJson() => _$SocketResponseToJson(this);
}

@JsonSerializable()
class WebSocketResponse {
  final String from;
  final String? method;
  final String? path;
  final dynamic data;

  WebSocketResponse({required this.from, this.method, this.path, this.data});

  factory WebSocketResponse.fromJson(Map<String, dynamic> json) =>
      _$WebSocketResponseFromJson(json);
  Map<String, dynamic> toJson() => _$WebSocketResponseToJson(this);
}

@JsonSerializable()
class WebSocketRequest {
  @JsonKey(name: 'type')
  final String type;
  final dynamic data;

  WebSocketRequest({required this.type, this.data});

  factory WebSocketRequest.fromJson(Map<String, dynamic> json) =>
      _$WebSocketRequestFromJson(json);
  Map<String, dynamic> toJson() => _$WebSocketRequestToJson(this);
}

@JsonSerializable()
class FilterType {
  final String type;
  final dynamic key;
  final dynamic value;
  final dynamic pattern;
  final FilterType? left;
  final FilterType? right;
  final FilterType? inner;

  FilterType({
    required this.type,
    this.key,
    this.value,
    this.pattern,
    this.left,
    this.right,
    this.inner,
  });

  factory FilterType.fromJson(Map<String, dynamic> json) =>
      _$FilterTypeFromJson(json);
  Map<String, dynamic> toJson() => _$FilterTypeToJson(this);
}

@JsonSerializable()
class SortType {
  final String type;
  final dynamic key;

  SortType({required this.type, required this.key});

  factory SortType.fromJson(Map<String, dynamic> json) =>
      _$SortTypeFromJson(json);
  Map<String, dynamic> toJson() => _$SortTypeToJson(this);
}

@JsonSerializable()
class Post {
  final int? id;
  final String title;
  final String content;

  Post({this.id, required this.title, required this.content});

  factory Post.fromJson(Map<String, dynamic> json) => _$PostFromJson(json);
  Map<String, dynamic> toJson() => _$PostToJson(this);
}

@JsonSerializable()
class Todo {
  final int id;
  final String task;
  final bool isCompleted;

  Todo({required this.id, required this.task, required this.isCompleted});

  factory Todo.fromJson(Map<String, dynamic> json) => _$TodoFromJson(json);
  Map<String, dynamic> toJson() => _$TodoToJson(this);
}

@JsonSerializable()
class Thread {
  final int id;
  final String title;
  final String author;
  final String contentId;
  final DateTime createdAt;
  final Content? content;

  Thread({
    required this.id,
    required this.title,
    required this.author,
    required this.contentId,
    required this.createdAt,
    this.content,
  });

  factory Thread.fromJson(Map<String, dynamic> json) => _$ThreadFromJson(json);

  Map<String, dynamic> toJson() => _$ThreadToJson(this);
}

@JsonSerializable(explicitToJson: true)
@ContentTypeConverter()
sealed class Content {
  const Content();
}

@JsonSerializable()
class TextContent extends Content {
  final String text;

  const TextContent({required this.text});

  factory TextContent.fromJson(Map<String, dynamic> json) =>
      _$TextContentFromJson(json);
  Map<String, dynamic> toJson() => _$TextContentToJson(this);
}

@JsonSerializable()
class ImageContent extends Content {
  final String imageUrl;

  const ImageContent({required this.imageUrl});

  factory ImageContent.fromJson(Map<String, dynamic> json) =>
      _$ImageContentFromJson(json);
  Map<String, dynamic> toJson() => _$ImageContentToJson(this);
}

@JsonSerializable()
class LinkContent extends Content {
  final String url;
  final String? preview;

  const LinkContent({required this.url, this.preview});

  factory LinkContent.fromJson(Map<String, dynamic> json) =>
      _$LinkContentFromJson(json);
  Map<String, dynamic> toJson() => _$LinkContentToJson(this);
}

class ContentTypeConverter
    implements JsonConverter<Content, Map<String, dynamic>> {
  const ContentTypeConverter();

  @override
  Content fromJson(Map<String, dynamic> json) {
    final type = json['type'];
    switch (type) {
      case 'text':
        return TextContent.fromJson(json);
      case 'image':
        return ImageContent.fromJson(json);
      case 'link':
        return LinkContent.fromJson(json);
      default:
        throw UnsupportedError('Unknown content type: $type');
    }
  }

  @override
  Map<String, dynamic> toJson(Content content) {
    final json = switch (content) {
      TextContent tc => tc.toJson()..['type'] = 'text',
      ImageContent ic => ic.toJson()..['type'] = 'image',
      LinkContent lc => lc.toJson()..['type'] = 'link',
    };
    return json;
  }
}
