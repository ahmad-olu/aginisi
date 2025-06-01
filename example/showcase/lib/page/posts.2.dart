import 'dart:async';
import 'dart:convert' show json;

import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:showcase/model/all.dart';
import 'package:web_socket_client/web_socket_client.dart';

//! websocket

class Post2Page extends HookWidget {
  const Post2Page({super.key});

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
    final channel = useMemoized(() {
      return WebSocket(Uri.parse('ws://localhost:8090/ws'));
    });
    final streamSnapshot = useStream(channel.messages);

    // When new data comes in, add it to messages list
    useEffect(() {
      if (streamSnapshot.hasData) {
        final d = json.decode(streamSnapshot.data) as Map<String, dynamic>;
        final path = d['path'] as String;
        final method = d['method'] as String;
        if (path == 'post') {
          final post = Post.fromJson(d['data']);
          // final post = streamSnapshot.data as String;
          posts.value = [...posts.value, post];
        }
      }
      return null;
    }, [streamSnapshot.data]);

    useEffect(() {
      return () {
        channel.close();
      };
    }, []);

    return Scaffold(
      appBar: AppBar(title: Text('Posts 1')),
      body: Column(
        children: [
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
