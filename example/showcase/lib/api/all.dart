import 'package:http/http.dart' as http;
import 'dart:convert';

import 'package:showcase/model/all.dart';

const baseurl = 'http://127.0.0.1:8090';

enum ReqMethod { get, post, patch, delete }

const contentT = ['application/json', 'application/x-www-form-urlencoded'];

Future<void> register(AuthInput authInput) async {
  final response = await _fetch(
    path: "auth/sign_up",
    body: authInput.toJson(),
    contentType: contentT[1],
  );

  if (response.statusCode == 200) {
  } else {
    throw Exception('Internal server error');
  }
}

Future<AuthBody> signInJwt(AuthInput authInput) async {
  final response = await _fetch(
    path: "auth/sign_in",
    body: authInput.toJson(),
    contentType: contentT[1],
  );

  if (response.statusCode == 200) {
    final data = json.decode(response.body) as Map<String, dynamic>;

    return AuthBody(
      tokenType: data['token_type'],
      accessToken: data['access_token'],
      refreshToken: data['refresh_token'],
    );
  } else {
    throw Exception('Internal server error');
  }
}

Future<(String, String)> signInSession(AuthInput authInput) async {
  final response = await _fetch(
    path: "auth/sign_in",
    body: authInput.toJson(),
    contentType: contentT[1],
  );

  if (response.statusCode == 200) {
    final data = json.decode(response.body) as Map<String, dynamic>;

    return (data['id'] as String, data['user_id'] as String);
  } else {
    throw Exception('Internal server error');
  }
}

Future<Data<Post>> createPost(Post postInput) async {
  final response = await _fetch(path: "post", body: postInput.toJson());

  if (response.statusCode == 200) {
    final data = json.decode(response.body) as Map<String, dynamic>;
    return Data.fromJson(data, (e) => Post.fromJson(e as Map<String, dynamic>));
  } else {
    throw Exception('Internal server error');
  }
}

Future<List> getPosts() async {
  final response = await _fetch(path: "post", method: ReqMethod.get);

  if (response.statusCode == 200) {
    final data = json.decode(response.body) as List<dynamic>;
    return data
        .map(
          (d) =>
              Data.fromJson(d, (e) => Post.fromJson(e as Map<String, dynamic>)),
        )
        .toList();
  } else {
    throw Exception('Internal server error');
  }
}

Future<Data<Post>> updatePost(Post postInput, int id) async {
  final response = await _fetch(
    path: "post/$id",
    body: postInput.toJson(),
    method: ReqMethod.patch,
  );

  if (response.statusCode == 200) {
    final data = json.decode(response.body) as Map<String, dynamic>;
    return Data.fromJson(data, (e) => Post.fromJson(e as Map<String, dynamic>));
  } else {
    throw Exception('Internal server error');
  }
}

Future<Data<Post>> deletePost(int id) async {
  final response = await _fetch(path: "post/$id", method: ReqMethod.delete);

  if (response.statusCode == 200) {
    final data = json.decode(response.body) as Map<String, dynamic>;
    return Data.fromJson(data, (e) => Post.fromJson(e as Map<String, dynamic>));
  } else {
    throw Exception('Internal server error');
  }
}

Future<http.Response> _fetch({
  required String path,
  ReqMethod? method = ReqMethod.post,
  Map<String, dynamic>? body,
  String? contentType,
}) async {
  final url = Uri.parse('$baseurl/$path');

  return switch (method) {
    ReqMethod.get => await http.get(
      url,
      headers: {'Content-Type': contentType ?? contentT[0]},
    ),

    ReqMethod.post || null => await http.post(
      url,
      headers: {'Content-Type': contentType ?? contentT[0]},
      body: jsonEncode(body),
    ),

    ReqMethod.patch => await http.patch(
      url,
      headers: {'Content-Type': contentT[0]},
      body: jsonEncode(body),
    ),

    ReqMethod.delete => await http.delete(
      url,
      headers: {'Content-Type': contentType ?? contentT[0]},
    ),
  };

  // if (response.statusCode == 200) {
  //   //ok
  //   final data = json.decode(response.body);
  // } else if (response.statusCode == 201) {
  //   //created
  // } else if (response.statusCode == 400) {
  //   //bad request
  // } else if (response.statusCode == 401) {
  //   //Unauthorized
  // } else if (response.statusCode == 403) {
  //   //forbidden
  // } else if (response.statusCode == 409) {
  //   //conflict
  // } else if (response.statusCode == 500) {
  //   //internal server error
  // } else {
  //   throw Error();
  // }
}

void getAccessToken() async {
  final refresh = '';
  await _fetch(path: 'refresh');
}
