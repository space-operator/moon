import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:plugin/generated/rid_api.dart';
import 'package:moon/layout.dart';
import 'package:dart_json_mapper/dart_json_mapper.dart';

import 'package:moon/rid/messaging.dart';
import 'package:moon/serialization/input_mapping.dart';
import 'package:moon/serialization/main.mapper.g.dart';
import 'package:path_provider/path_provider.dart';

void main() async {
  RidMessaging.init();

  rid.debugLock = null;
  rid.debugReply = null;
  //rid.debugReply = (reply) => debugPrint('$reply');

  WidgetsFlutterBinding.ensureInitialized();

  // MUST DO DEVICE TYPE CHECK AND UPDATE
  //final libraryDir =
  await getLibraryDirectory(); // Unsupported on Android and Linux
  //print(libraryDir);
  final tempDir = await getTemporaryDirectory();
  final persistentDir = await getApplicationSupportDirectory();
  // print(persistentDir);

  final docDir = await getApplicationDocumentsDirectory();
  // print(docDir);
  initializeJsonMapper();

  final initEvent = {
    "dbPath": persistentDir.path,
    "logPath": persistentDir.path,
    "canvasWidth": 1920, // Screen size
    "canvasHeight": 950,
  };

  String event = JsonMapper.serialize(InputProperties(initEvent));

  await Store.instance.msgInitialize(event); /*initializeEvent*/

  // rid_ffi.rid_export_query_graph();
  // rid_ffi.rid_export_start();
  // rid_ffi.rid_export_handle_input2(1);

  runApp(ProviderScope(child: MyApp()));
  // print(
  //   JsonMapper.serialize(
  //     InputProperties({"propertyName": "property"}),
  //   ),
  // );
}

class MyApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
        debugShowCheckedModeBanner: false,
        title: 'Space Operator',
        theme: ThemeData(
          fontFamily: 'Inter',
          primarySwatch: Colors.lightBlue,
          visualDensity: VisualDensity.adaptivePlatformDensity,
        ),
        home: LayoutScreen(
            // key: UniqueKey(),
            ));
  }
}

// /// https://blog.gskinner.com/archives/2020/03/flutter-simplify-platform-detection-responsive-sizing.html
// class Screen {
//   static double get _ppi => (Platform.isAndroid || Platform.isIOS) ? 150 : 96;
//   static bool isLandscape(BuildContext c) =>
//       MediaQuery.of(c).orientation == Orientation.landscape;
//   //PIXELS
//   static Size size(BuildContext c) => MediaQuery.of(c).size;
//   static double width(BuildContext c) => size(c).width;
//   static double height(BuildContext c) => size(c).height;
//   static double diagonal(BuildContext c) {
//     Size s = size(c);
//     return sqrt((s.width * s.width) + (s.height * s.height));
//   }

//   //INCHES
//   static Size inches(BuildContext c) {
//     Size pxSize = size(c);
//     return Size(pxSize.width / _ppi, pxSize.height / _ppi);
//   }

//   static double widthInches(BuildContext c) => inches(c).width;
//   static double heightInches(BuildContext c) => inches(c).height;
//   static double diagonalInches(BuildContext c) => diagonal(c) / _ppi;
// }
