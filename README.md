# music-chat-tracker

install rust
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
install bazel https://bazel.build/install

extensions
```
Bazel
rust-analyzer
```

bazel version, maybe
```
Bazelisk version: development
Build label: 7.2.1
Build target: @@//src/main/java/com/google/devtools/build/lib/bazel:BazelServer
Build time: Tue Jun 25 15:53:57 2024 (1719330837)
Build timestamp: 1719330837
Build timestamp as int: 1719330837
```

### Running it
build it
```
bazel build //:music_chat_tracker
```
test it
```
bazel test //:music_chat_tracker
```
run it
```
bazel run //:music_chat_tracker
```
ezpz