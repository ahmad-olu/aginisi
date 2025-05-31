import 'dart:math';
import 'package:showcase/model/all.dart';
import 'package:uuid/uuid.dart';

final _uuid = Uuid();
final _random = Random();

List<String> sampleTitles = [
  'What do you think about Rust?',
  'Flutter vs React Native',
  'Daily Coding Challenge',
  'Share your favorite tool!',
  'Any good Linux tips?',
];

List<String> sampleAuthors = [
  'alice42',
  'bob_dev',
  'charlie',
  'deltaX',
  'tech_guru',
];

Thread generateRandomThread() {
  return Thread(
    id: _uuid.v4(),
    title: sampleTitles[_random.nextInt(sampleTitles.length)],
    author: sampleAuthors[_random.nextInt(sampleAuthors.length)],
    contentId: _uuid.v4(),
    createdAt: DateTime.now().subtract(
      Duration(minutes: _random.nextInt(10000)),
    ),
    content: null, // Excluded as requested
  );
}
