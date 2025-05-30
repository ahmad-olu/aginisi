import 'package:flutter/material.dart';
import 'package:routemaster/routemaster.dart';

class HomePage extends StatelessWidget {
  const HomePage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Home'),
        centerTitle: true,

        backgroundColor: Colors.green[50],
      ),
      body: SingleChildScrollView(
        child: Column(
          children: [
            ListItem(
              title: 'sign up',
              onTap: () => Routemaster.of(context).push('/sign-up'),
            ),
            ListItem(title: 'sign in'),
            ListItem(title: 'posts 1'),
            ListItem(title: 'posts 2'),
            ListItem(title: 'posts 3'),
            ListItem(title: 'posts 4'),
            ListItem(title: 'file upload'),
          ],
        ),
      ),
    );
  }
}

class ListItem extends StatelessWidget {
  const ListItem({super.key, required this.title, this.subTitle, this.onTap});
  final String title;
  final String? subTitle;
  final void Function()? onTap;

  @override
  Widget build(BuildContext context) {
    return InkWell(
      onTap: onTap,
      child: Container(
        height: 80,
        decoration: BoxDecoration(),
        child: Center(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              Text(title, style: TextStyle().copyWith(fontSize: 25)),
              Text(subTitle ?? ''),
            ],
          ),
        ),
      ),
    );
  }
}
