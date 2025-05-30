import 'package:flutter/material.dart';
import 'package:routemaster/routemaster.dart';
import 'package:showcase/main.dart';
import 'package:showcase/page/auth.register.dart';
import 'package:showcase/page/auth.sign_in.dart';
import 'package:showcase/page/file_upload.dart';
import 'package:showcase/page/home.dart';
import 'package:showcase/page/posts.1.dart';
import 'package:showcase/page/posts.2.dart';
import 'package:showcase/page/posts.3.dart';
import 'package:showcase/page/posts.4.dart';

final routes = RouteMap(
  routes: {
    '/': (_) => MaterialPage(child: HomePage()),
    '/sign-in': (_) => MaterialPage(child: SignInPage()),
    '/sign-up': (_) => MaterialPage(child: RegisterPage()),
    '/posts-1':
        (_) =>
            !canAccessPage()
                ? Redirect('/')
                : MaterialPage(child: Posts1Page()),
    '/posts-2':
        (_) =>
            !canAccessPage()
                ? Redirect('/')
                : MaterialPage(child: Posts2Page()),
    '/posts-3':
        (_) =>
            !canAccessPage()
                ? Redirect('/')
                : MaterialPage(child: Posts3Page()),

    '/posts-4/:type':
        (info) =>
            !canAccessPage()
                ? Redirect('/')
                : MaterialPage(
                  child: Posts4Page(type: info.pathParameters['type']!),
                ),
    '/upload':
        (_) =>
            !canAccessPage()
                ? Redirect('/')
                : MaterialPage(child: FileUploadPage()),
  },
);

bool canAccessPage() {
  if (authenticationType == AuthTYpe.none) {
    return true;
  } else if (authenticationType == AuthTYpe.jwt) {
    return true;
  } else if (authenticationType == AuthTYpe.session) {
    return true;
  }

  return true;
}

//Routemaster.of(context).push('/feed/profile/1');
