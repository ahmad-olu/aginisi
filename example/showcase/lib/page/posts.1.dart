import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:showcase/api/all.dart';
import 'package:showcase/model/all.dart';

//! get method

class Posts1Page extends HookWidget {
  const Posts1Page({super.key});

  @override
  Widget build(BuildContext context) {
    final revalidate = useState(0);
    final form = useMemoized(GlobalKey<FormState>.new, []);
    final titleController = useTextEditingController();
    final contentController = useTextEditingController();
    final isUploading = useState(false);

    final postsM = useMemoized(() => getPosts(), [revalidate.value]);
    final posts = useFuture(postsM);
    if (posts.hasError) {
      return Center(child: Text(posts.error.toString()));
    }

    return Scaffold(
      appBar: AppBar(title: Text('Posts 1')),
      body: Column(
        children: [
          if (isUploading.value)
            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 50, vertical: 10),
              child: LinearProgressIndicator(),
            ),
          if (posts.data == null || posts.data!.isEmpty) Text('No Post yet'),
          Expanded(
            child: ListView.builder(
              itemCount: posts.data!.length,
              itemBuilder: (context, index) {
                final post = posts.data![index];
                return ListTile(
                  trailing: Text(post.data!.id.toString()),
                  title: Text(post.data!.title),
                  subtitle: Text(post.data!.content),
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
