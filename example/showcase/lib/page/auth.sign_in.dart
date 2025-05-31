import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:routemaster/routemaster.dart';
import 'package:showcase/api/all.dart';
import 'package:showcase/model/all.dart';

class SignInPage extends HookWidget {
  const SignInPage({super.key});

  @override
  Widget build(BuildContext context) {
    final form = useMemoized(GlobalKey<FormState>.new, []);
    final emailController = useTextEditingController();
    final passwordController = useTextEditingController();
    final isSubmitting = useState(false);
    return Scaffold(
      appBar: AppBar(),
      body: Padding(
        padding: const EdgeInsets.all(20.0),
        child: Form(
          key: form,
          child: Column(
            children: [
              Text('Sign In'),

              TextFormField(
                controller: emailController,
                decoration: InputDecoration(helper: Text('email')),
                validator: (v) {
                  if (v == null || v.isEmpty) {
                    return 'Field cant be empty';
                  }
                  return null;
                },
              ),
              TextFormField(
                controller: passwordController,
                // readOnly: true,
                obscureText: true,
                decoration: InputDecoration(helper: Text('[password]')),
                validator: (v) {
                  if (v == null || v.isEmpty) {
                    return 'Field cant be empty';
                  }
                  return null;
                },
              ),

              if (isSubmitting.value)
                Padding(
                  padding: const EdgeInsets.symmetric(horizontal: 15),
                  child: LinearProgressIndicator(),
                )
              else
                Column(
                  children: [
                    ElevatedButton(
                      onPressed: () async {
                        try {
                          if (form.currentState!.validate()) {
                            isSubmitting.value = true;
                            final res = await signInJwt(
                              AuthInput(
                                email: emailController.text,
                                password: passwordController.text,
                              ),
                            );
                            isSubmitting.value = false;
                            if (res == true) {
                              if (!context.mounted) {
                                return;
                              }
                              Routemaster.of(context).push('/');
                            }
                          }
                        } catch (e) {
                          isSubmitting.value = false;
                        }
                      },
                      child: Text('Sign in Jwt'),
                    ),
                    SizedBox(height: 12),
                    ElevatedButton(
                      onPressed: () async {
                        try {
                          if (form.currentState!.validate()) {
                            isSubmitting.value = true;
                            final res = await signInSession(
                              AuthInput(
                                email: emailController.text,
                                password: passwordController.text,
                              ),
                            );
                            isSubmitting.value = false;
                            if (res == true) {
                              if (!context.mounted) {
                                return;
                              }
                              Routemaster.of(context).push('/');
                            }
                          }
                        } catch (e) {
                          isSubmitting.value = false;
                        }
                      },
                      child: Text('Sign in Session'),
                    ),
                  ],
                ),
            ],
          ),
        ),
      ),
    );
  }
}
