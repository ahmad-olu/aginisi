import 'package:flutter/material.dart';
import 'package:routemaster/routemaster.dart';
import 'package:showcase/route/route.dart';

void main() {
  runApp(const MainApp());
}

enum AuthTYpe { jwt, session, none }

const authenticationType = AuthTYpe.none;

class MainApp extends StatelessWidget {
  const MainApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp.router(
      routerDelegate: RoutemasterDelegate(routesBuilder: (context) => routes),
      routeInformationParser: RoutemasterParser(),
    );
  }
}
