import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:plugin/generated/rid_api.dart' as rid;

import 'package:flutter/material.dart';
import 'package:moon/providers/store_provider.dart';
import 'package:tuple/tuple.dart';

class InputDisk extends HookConsumerWidget {
  const InputDisk({
    Key? key,
    required this.nodeEntry,
  }) : super(key: key);

  final Tuple2<String, rid.NodeView> nodeEntry;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    // print("// rebuilding inputdisk ${key}");
    // rebuild if highlighted
    String? highlighted = ref.watch(highlightedPort.select((list) {
      final match = list.where((element) => element == nodeEntry.item1);
      if (match.isNotEmpty) {
        // print("is highlighted");
        return match.first;
      }
    }));

    List<String> edges = [];

    bool currentlyDragged = false;

    ref.watch(nodeController.select((map) {
      final mapList = map.entries.where(
        (entry) => entry.key == nodeEntry.item1,
      );
      if (mapList.isNotEmpty) {
        // print("get inbound edges");
        edges = mapList.first.value.flowInboundEdges;

        final dummy = ref
            .read(edgeController)
            .entries
            .where((element) => element.key == "dummy_edge");

        if (dummy.isNotEmpty && dummy.first.value.to == nodeEntry.item1)
          currentlyDragged = true;
        return mapList.first.value.flowInboundEdges;
      }
    }));

//
    return Container(
      width: 30,
      child: edges.length <= 1
          ? ElevatedButton(
              onHover: (value) {},
              // color:
              //     edges.contains(nodeEntry.key) ? Colors.green : Colors.white,
              onPressed: () {},
              style: ElevatedButton.styleFrom(
                primary: edges.length == 1 || currentlyDragged == true
                    ? Colors.amber
                    : highlighted != null
                        ? Color.fromARGB(255, 168, 216, 114)
                        : Colors.white,
                fixedSize: const Size(30, 30),
                shape: CircleBorder(
                  side: BorderSide(
                      color: edges.length == 1 ||
                              currentlyDragged != false ||
                              !nodeEntry.item2.required
                          ? Colors.black26
                          : Colors.red,
                      style: BorderStyle.solid,
                      width: edges.length == 1 ||
                              currentlyDragged != false ||
                              !nodeEntry.item2.required
                          ? 1
                          : 3),
                ),
              ),
              child: edges.length > 1
                  ? Text(
                      edges.length.toString(),
                      textAlign: TextAlign.center,
                    )
                  : null,
            )
          : Tooltip(
              decoration: BoxDecoration(
                borderRadius: BorderRadius.circular(5),
                color: Colors.red,
              ),
              height: 50,
              padding: const EdgeInsets.all(8.0),
              preferBelow: false,
              textStyle: const TextStyle(fontSize: 24, color: Colors.white),
              // showDuration: const Duration(seconds: 2),
              // waitDuration: const Duration(seconds: 0),
              message: "multiple edges not yet supported, remove one edge",
              child: ElevatedButton(
                onHover: (value) {},
                // color:
                //     edges.contains(nodeEntry.key) ? Colors.green : Colors.white,
                onPressed: () {},
                style: ElevatedButton.styleFrom(
                  primary: Colors.red,
                  fixedSize: const Size(30, 30),
                  shape: const CircleBorder(
                    side: const BorderSide(
                      color: Colors.black26,
                      style: BorderStyle.solid,
                    ),
                  ),
                ),
                child: Text(
                  edges.length.toString(),
                  textAlign: TextAlign.center,
                ),
              ),
            ),
    );
  }
}
