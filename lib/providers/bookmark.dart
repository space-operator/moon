import 'dart:collection';
import 'dart:async';
import 'dart:io';
import 'dart:typed_data';

import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:path_provider/path_provider.dart';
import 'package:plugin/generated/rid_api.dart';
import 'package:moon/providers/store_provider.dart';

import 'package:screenshot/screenshot.dart';

final screenshotController =
    Provider<ScreenshotController>(((ref) => ScreenshotController()));

final viewportController =
    StateNotifierProvider<ViewportController, List<Camera>>(
        (ref) => ViewportController(ref));

class ViewportController extends StateNotifier<List<Camera>> {
  final Ref _ref;

  ViewportController(this._ref) : super([]) {
    // _subscribe();
  }

  updateState() {
    // print("update viewportController");
    // state = _ref.refresh(lastChangesRepoProvider); // refreshes too often
    state = [_ref.read(storeRepoProvider).transform];
  }

  updateToScreenshot() {
    state = [_ref.read(storeRepoProvider).transformScreenshot];
  }
}

final bookmarkController =
    StateNotifierProvider<BookmarkController, HashMap<String, BookmarkView>>(
        (ref) => BookmarkController(ref));

class BookmarkController extends StateNotifier<HashMap<String, BookmarkView>> {
  final Ref _ref;

  BookmarkController(this._ref) : super(HashMap<String, BookmarkView>()) {
    updateState();
  }

  updateState() {
    // print("update bookmarkState");
    state = _ref.read(storeRepoProvider).bookmarks;
  }
}

Future<String> getFilePath() async {
  Directory appDocumentsDirectory =
      await getApplicationDocumentsDirectory(); // 1
  String appDocumentsPath = appDocumentsDirectory.path; // 2
  String filePath = '$appDocumentsPath/demoTextFile.png'; // 3

  return filePath;
}

void saveFile(bytes) async {
  File file = File(await getFilePath()); // 1
  // print(file.path);
  file.writeAsBytes(bytes);
}

final transformScreenshotController =
    StateNotifierProvider<TransformScreenshotController, List<Camera>>(
        (ref) => TransformScreenshotController(ref));

class TransformScreenshotController extends StateNotifier<List<Camera>> {
  final Ref _ref;

  TransformScreenshotController(this._ref) : super([]) {
    // _subscribe();
  }

  screenshot() {
    // print("update viewportController");
    // state = _ref.refresh(lastChangesRepoProvider); // refreshes too often
    state = [_ref.read(storeRepoProvider).transformScreenshot];

    // print(state);
    BuildContext context = _ref.read(contextProvider).context!;
    // print(context);

    takeScreenshot(context);

    //restore transform
  }

//_ref.read(viewportController.notifier).updateState()
  Future<dynamic> ShowCapturedWidget(
    BuildContext context,
    Uint8List capturedImage,
  ) {
    return showDialog(
      useSafeArea: false,
      context: context,
      builder: (context) => Scaffold(
        backgroundColor: Colors.blueGrey[900],
        appBar: AppBar(
          backgroundColor: Colors.blueGrey,
          title: const Text(
            "Bookmarked Node",
          ),
        ),
        body: Center(
            child: Image.memory(
          capturedImage,
        )),
      ),
    );
  }

  takeScreenshot(context) {
    _ref
        .read(screenshotController)
        .capture(
          delay: const Duration(milliseconds: 500),
        )
        .then((capturedImage) {
      // saveFile(capturedImage);
      ShowCapturedWidget(context, capturedImage!);
    }).whenComplete(() {
      Future.delayed(Duration(seconds: 1), () {
        _ref.read(viewportController.notifier).updateState();
      });
    });

    ;
  }

  //  takeScreenshot(context) {
  //   _ref
  //       .read(screenshotController)
  //       .captureFromWidget(
  //           InheritedTheme.captureAll(
  //               context,
  //               Material(
  //                   child: ProviderScope(
  //                       child: MaterialApp(
  //                           theme: ThemeData(
  //                             primarySwatch: Colors.lightBlue,
  //                             visualDensity:
  //                                 VisualDensity.adaptivePlatformDensity,
  //                           ),
  //                           debugShowCheckedModeBanner: false,
  //                           home: CanvasScreenshot(transform: state[0]))))),
  //           delay: Duration(seconds: 1))
  //       .then((capturedImage) {
  //     ShowCapturedWidget(context, capturedImage);
  //   });
  // }
}
