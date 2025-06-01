import 'dart:async';
import 'dart:convert' show json;
import 'dart:developer';

import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:showcase/api/all.dart';
import 'package:showcase/model/all.dart';
import 'package:socket_io_client/socket_io_client.dart' as io;

// ! socket io

class Post3Page extends HookWidget {
  const Post3Page({super.key});

  @override
  Widget build(BuildContext context) {
    final revalidate = useState(0);
    final form = useMemoized(GlobalKey<FormState>.new, []);
    final titleController = useTextEditingController();
    final contentController = useTextEditingController();
    final posts = useState<List<Post>>([]);
    final isUploading = useState(false);

    // final postsM = useMemoized(() => getPosts(), [revalidate.value]);
    // final posts = useFuture(postsM);
    final postController = useMemoized(
      () => StreamController<(String, Post)>(),
      [],
    );
    final socket = useMemoized<io.Socket>(() {
      final s = io.io(
        'http://localhost:8090/socket', // ✔️
        io.OptionBuilder().setTransports([
          'websocket',
        ]) // Ensure WebSocket is used
        .build(),
      );

      s.on('post-listener', (response) {
        log('Received message: $response');
        final d = json.decode(response) as Map<String, dynamic>;
        final method = d['method'] as String;
        final data = Post.fromJson(d);
        postController.add((method, data));
        posts.value = [...posts.value, data];
      });
      s.onDisconnect((_) {
        log('Disconnected from server');
      });
      return s;
    }, []);

    useEffect(() {
      return () {
        socket.dispose();
        postController.close();
        log('Socket & controller disposed');
      };
    }, []);

    return Scaffold(
      appBar: AppBar(title: Text('Posts 1')),
      body: Column(
        children: [
          SizedBox(
            height: 100,
            child: StreamBuilder<(String, Post)>(
              stream: postController.stream,
              builder: (context, snapshot) {
                if (snapshot.hasError) {
                  return Center(child: Text('Error: ${snapshot.error}'));
                }
                // if (snapshot.connectionState == ConnectionState.waiting) {}
                final post = snapshot.data;
                return Text('${post!.$2.title} ==> ${post.$2.content}');
              },
            ),
          ),
          Expanded(
            child: ListView.builder(
              itemCount: posts.value.length,
              itemBuilder: (context, index) {
                final post = posts.value[index];
                return ListTile(
                  trailing: Text(post.id.toString()),
                  title: Text(post.title),
                  subtitle: Text(post.content),
                );
              },
            ),
          ),
          Form(
            key: form,
            child: Padding(
              padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 10),
              child: Column(
                children: [
                  TextFormField(
                    controller: titleController,
                    decoration: InputDecoration(helper: Text('title')),
                  ),
                  SizedBox(height: 5),
                  TextFormField(
                    controller: contentController,
                    decoration: InputDecoration(helper: Text('content')),
                  ),
                  SizedBox(height: 5),
                  ElevatedButton(
                    onPressed: () async {
                      try {
                        if (form.currentState!.validate()) {
                          isUploading.value = true;
                          await createPost(
                            Post(
                              title: titleController.text,
                              content: contentController.text,
                            ),
                          );
                          titleController.clear();
                          contentController.clear();
                          revalidate.value++;
                          isUploading.value = false;
                        }
                      } catch (e) {
                        isUploading.value = false;
                      }
                    },
                    child: Text('submit'),
                  ),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }
}
