import 'package:http/http.dart' as http;
//import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import 'package:localstorage/localstorage.dart';

import 'dart:convert';

import 'package:showcase/model/all.dart';

const baseurl = 'http://127.0.0.1:8090';

enum ReqMethod { get, post, patch, delete }

const contentT = ['application/json', 'application/x-www-form-urlencoded'];

Future<bool> register(AuthInput authInput) async {
  try {
    final response = await _fetch(
      path: "auth/sign_up",
      body: authInput.toJson(),
      contentType: contentT[1],
    );

    if (response.statusCode == 200) {
      return true;
    } else {
      throw Exception('Internal server error');
    }
  } catch (e) {
    return false;
  }
}

Future<bool> signInJwt(AuthInput authInput) async {
  try {
    final body = authInput.toJson()..remove("name");
    final response = await _fetch(
      path: "auth/sign_in",
      body: body,
      // body: {"email": authInput.email, "password": authInput.password},
      contentType: contentT[1],
    );
    if (response.statusCode == 200) {
      final data = json.decode(response.body) as Map<String, dynamic>;

      // final a = AuthBody(
      //   tokenType: data['token_type'],
      //   accessToken: data['access_token'],
      //   refreshToken: data['refresh_token'],
      // );
      // final storage = FlutterSecureStorage();
      // await storage.write(key: 'access_token', value: data['access_token']);
      // await storage.write(key: 'refresh_token', value: data['refresh_token']);
      localStorage.setItem('access_token', data['access_token']);
      localStorage.setItem('refresh_token', data['refresh_token'] ?? '');
      return true;
    } else {
      throw Exception('Internal server error');
    }
  } catch (e) {
    print('error ==> $e');
    return false;
  }
}

Future<bool> signInSession(AuthInput authInput) async {
  final response = await _fetch(
    path: "auth/sign_in",
    body: authInput.toJson(),
    contentType: contentT[1],
  );

  if (response.statusCode == 200) {
    final data = json.decode(response.body) as Map<String, dynamic>;

    // return (data['id'] as String, data['user_id'] as String);
    // final storage = FlutterSecureStorage();
    // await storage.write(key: 'x-session', value: data['id']);

    localStorage.setItem('x-session', data['id']);
    print('error');
    return true;
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
      //   body: jsonEncode(body),
      body: body,
    ),

    ReqMethod.patch => await http.patch(
      url,
      headers: {'Content-Type': contentT[0]},
      body: body,
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
