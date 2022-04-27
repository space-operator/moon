import 'package:dart_json_mapper/dart_json_mapper.dart';
import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:moon/bookmark_manager.dart';
import 'package:moon/drawer.dart';
import 'package:moon/guide_manager.dart';
import 'package:moon/utils/logger.dart';
import 'package:moon/providers/store_provider.dart';
import 'package:plugin/generated/rid_api.dart' as rid;
import 'package:moon/serialization/input_mapping.dart';

import 'package:moon/title_area.dart';
import 'package:url_launcher/url_launcher.dart';

import 'event_listener.dart';

class LayoutScreen extends HookConsumerWidget {
  static const routeName = "/Create";
  const LayoutScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext buildContext, WidgetRef ref) {
    ValueNotifier<bool> splashScreen = useState(true);

    // print("rebuilding layout");

    // print("rebuilding layout");
    // var _last = useState("");
    final provider = ref.watch(changesController);
    final graphProvider = ref.watch(graphController);
    final selected_node_ids =
        ref.watch(selectedNodeIds.select((value) => value.length));

    final store = ref.read(storeRepoProvider).store;
    final graph_entry = ref.read(storeRepoProvider).graph_entry;

    // create dropdown options
    final List<rid.GraphEntry> graphList =
        ref.read(storeRepoProvider).graph_list;

    final showButtonText =
        MediaQuery.of(buildContext).size.width > 1600 ? true : false;
    final showBookmark =
        MediaQuery.of(buildContext).size.width > 1200 ? true : false;
    // create dropdown options
    final List<String> mainnet = const ["Testnet", "Devnet", "Mainnet"];

    List<DropdownMenuItem<String>> mainnetList = mainnet.map(
      (e) {
        return DropdownMenuItem(child: Text(e), value: e);
      },
    ).toList();

    final mainnetSelection = useState(store.view.solanaNet.name.toString());

    ///
    final hideDrawer = useState(true);

    final GlobalKey<ScaffoldState> _scaffoldKey = GlobalKey();

    /// Text renaming
    final renameTextEditingController =
        useTextEditingController(text: graph_entry.name);

    final graph_name = ValueNotifier(graph_entry.name);

    final update = useValueListenable(graph_name);

    useEffect(() {
      renameTextEditingController.text = update;
    }, [update]);

    final debug = useState("");

    // bookmark
    // final selected_nodes_ids = ref.read(storeRepoProvider).selected_node_ids;

    final bookmarkTextController = useTextEditingController(text: "");

    final menuScroll = useScrollController();
    final debugScrollController = useScrollController();

    return Scaffold(
      key: _scaffoldKey,
      drawer: DrawerWidget(
          key: ObjectKey(graph_entry.name),
          menuScroll: menuScroll,
          store: store,
          debugScrollController: debugScrollController,
          debug: debug,
          renameTextEditingController: renameTextEditingController,
          graph_entry: graph_entry,
          graphList: graphList),
      appBar: AppBar(
        leading: Builder(
          builder: (BuildContext context) {
            return IconButton(
              icon: const Icon(Icons.menu, color: Colors.white),
              onPressed: () {
                Scaffold.of(context).openDrawer();
              },
              tooltip: MaterialLocalizations.of(context).showMenuTooltip,
            );
          },
        ),
        backgroundColor: Colors.black87,
        title: const TitleArea(),
        actions: [
          // DebugWidget(),
          if (showBookmark)
            TextButton.icon(
                style: TextButton.styleFrom(
                  primary: Colors.blueGrey.shade400,
                ),
                icon: const Icon(Icons.bookmark_add_outlined),
                onPressed: () {
                  if (bookmarkTextController.value.text != "" &&
                      selected_node_ids != 0) {
                    ref.read(contextController.notifier).update(buildContext);
                    store
                        .msgCreateBookmark(bookmarkTextController.value.text)
                        .then(
                          (value) => store.msgBookmarkScreenshot(value.data!,
                              timeout: Duration(minutes: 1)),
                        );
                    bookmarkTextController.value = TextEditingValue.empty;
                  }
                },
                label: Text("")),
          if (showBookmark)
            Container(
                width: 150,
                child: TextField(
                  decoration: InputDecoration(
                    label: Text(
                      "enter bookmark name",
                      style: TextStyle(
                          color: Colors.blueGrey.shade400, fontSize: 12),
                    ),
                    // hintText: "bookmark name",
                    // hintStyle: TextStyle(color: Colors.white, fontSize: 10),
                  ),
                  controller: bookmarkTextController,
                  style: TextStyle(color: Colors.blueGrey.shade400),
                  onSubmitted: (value) {
                    if (value != "" && selected_node_ids != 0) {
                      ref.read(contextController.notifier).update(buildContext);
                      store
                          .msgCreateBookmark(bookmarkTextController.value.text,
                              timeout: Duration(minutes: 1))
                          .then(
                            (value) => store.msgBookmarkScreenshot(value.data!,
                                timeout: Duration(minutes: 1)),
                          );
                      bookmarkTextController.value = TextEditingValue.empty;
                    }
                  },
                )),
          if (showButtonText)
            Padding(
              padding: const EdgeInsets.all(8.0),
              child: const VerticalDivider(color: Colors.white),
            ),
          Padding(
            padding: const EdgeInsets.all(8.0),
            child: DropdownButton(
                style: const TextStyle(
                    fontSize: 18, color: Color.fromARGB(255, 243, 170, 33)),
                alignment: Alignment.bottomCenter,
                items: mainnetList,
                value: mainnetSelection.value,
                onChanged: (value) {
                  mainnetSelection.value = value.toString();
                  store.msgChangeSolanaNet(value.toString(),
                      timeout: Duration(minutes: 1));
                }),
          ),
          TextButton.icon(
              style: TextButton.styleFrom(
                primary: Colors.blueGrey.shade400,
              ),
              icon: const Icon(Icons.play_arrow_rounded),
              onPressed: () {
                store.msgDeploy("deploy", timeout: Duration(minutes: 120));
              },
              label: showButtonText ? const Text("Deploy") : Text("")),
          TextButton.icon(
              style: TextButton.styleFrom(
                primary: Colors.blueGrey.shade400,
              ),
              icon: const Icon(Icons.stop),
              onPressed: () {
                store.msgUnDeploy("undeploy", timeout: Duration(minutes: 1));
              },
              label: showButtonText ? const Text("UnDeploy") : Text("")),
          const Padding(
            padding: const EdgeInsets.all(8.0),
            child: const VerticalDivider(color: Colors.white),
          ),
          TextButton.icon(
            style: TextButton.styleFrom(
              primary: Colors.blueGrey.shade400,
            ),
            icon: const Icon(Icons.fit_screen),
            label: showButtonText ? const Text("fit to screen") : Text(""),
            onPressed: () {
              ref.read(storeRepoProvider).store.msgFitNodesToScreen(
                  "fit to screen",
                  timeout: Duration(minutes: 1));
              // _transformationController.value = Matrix4.identity();
            },
          ),
          TextButton.icon(
            style: TextButton.styleFrom(
              primary: Colors.blueGrey.shade400,
            ),
            icon: const Icon(Icons.restart_alt_rounded),
            label: showButtonText ? const Text("reset zoom") : Text(""),
            onPressed: () {
              ref
                  .read(storeRepoProvider)
                  .store
                  .msgResetZoom("reset zoom", timeout: Duration(minutes: 1));
              // _transformationController.value = Matrix4.identity();
            },
          ),
          IconButton(
            icon: const Icon(Icons.zoom_in),
            color: Colors.blueGrey.shade400,
            onPressed: () {
              ref
                  .read(storeRepoProvider)
                  .store
                  .msgZoomIn("", timeout: Duration(minutes: 1));
              // _transformationController.value = Matrix4.identity();
            },
          ),
          IconButton(
            icon: const Icon(Icons.zoom_out),
            color: Colors.blueGrey.shade400,
            onPressed: () {
              ref
                  .read(storeRepoProvider)
                  .store
                  .msgZoomOut("", timeout: Duration(minutes: 1));
              // _transformationController.value = Matrix4.identity();
            },
          ),
        ],
      ),
      body: Stack(children: [
        LayoutBuilder(
          builder: (context, constraints) {
            final resizeEvent = {
              "width": constraints.maxWidth.toInt(), // Screen size
              "height": constraints.maxHeight.toInt(),
            };
            final String event =
                JsonMapper.serialize(InputProperties(resizeEvent));
            ref
                .read(storeRepoProvider)
                .store
                .msgResizeCanvas(event, timeout: Duration(seconds: 20));
            return Container(
              // height: MediaQuery.of(context).size.height + height,
              // width: MediaQuery.of(context).size.width + width,
              decoration: BoxDecoration(color: Colors.blueGrey[900]),
              child: EventListener(),
            );
          },
        ),
        Positioned(
          bottom: 0,
          right: 0,
          child: Image.asset(
            "assets/logo-full-small.png",
            height: 75,
          ),
        ),
        Row(
          children: [
            if (!hideDrawer.value)
              Column(
                children: [const BookmarkManager(), const GuideManager()],
              ),
            GestureDetector(
              behavior: HitTestBehavior.opaque,
              onTap: () {
                hideDrawer.value = !hideDrawer.value;
              },
              child: Container(
                  width: 30,
                  decoration: BoxDecoration(
                      color: Colors.grey[900],
                      border: Border.all(color: Colors.black26)),
                  child: Column(
                    children: [
                      Expanded(child: Container()),
                      !hideDrawer.value
                          ? const Icon(
                              Icons.keyboard_arrow_left_rounded,
                              color: Colors.white,
                            )
                          : const Icon(Icons.keyboard_arrow_right_rounded,
                              color: Colors.white),
                      Expanded(child: Container()),
                    ],
                  )),
            )
          ],
        )
      ]),
      // ),
    );
  }
}

class DebugWidget extends HookConsumerWidget {
  const DebugWidget({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final provider = ref.watch(debugController);

    return Container(
      child: Column(children: [
        Text(ref.read(storeRepoProvider).debugData.mappingKind,
            style: TextStyle(color: Colors.white)),
        Text(ref.read(storeRepoProvider).debugData.uiState,
            style: TextStyle(color: Colors.white)),
        Text(ref.read(storeRepoProvider).debugData.selectedNodeIds,
            style: TextStyle(color: Colors.white)),
      ]),
    );
  }
}


// final TransformationController _transformationController =
  //     TransformationController();

  // double height = 3000;
  // double width = 3000;

  // https://api.flutter.dev/flutter/widgets/InteractiveViewer/transformationController.html
  // Animation<Matrix4>? _animationReset;
  // late final AnimationController _controllerReset;

  // void _onAnimateReset() {
  //   _transformationController.value = _animationReset!.value;
  //   if (!_controllerReset.isAnimating) {
  //     _animationReset!.removeListener(_onAnimateReset);
  //     _animationReset = null;
  //     _controllerReset.reset();
  //   }
  // }

  // void _onInteractionStart(ScaleStartDetails details) {
  //   // If the user tries to cause a transformation while the reset animation is
  //   // running, cancel the reset animation.
  //   if (_controllerReset.status == AnimationStatus.forward) {
  //     _animateResetStop();
  //   }
  // }

  // void _animateResetInitialize() {
  //   _controllerReset.reset();
  //   _animationReset = Matrix4Tween(
  //     begin: _transformationController.value,
  //     end: Matrix4.identity(),
  //   ).animate(_controllerReset);
  //   _animationReset!.addListener(_onAnimateReset);
  //   _controllerReset.forward();
  // }

  // // Stop a running reset to home transform animation.
  // void _animateResetStop() {
  //   _controllerReset.stop();
  //   _animationReset?.removeListener(_onAnimateReset);
  //   _animationReset = null;
  //   _controllerReset.reset();
  // }