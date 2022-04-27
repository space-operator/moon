import 'dart:collection';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:plugin/generated/rid_api.dart' as rid;

import 'package:flutter/material.dart';
import 'package:moon/nodes/input_disk.dart';
import 'package:moon/nodes/port_entry.dart';
import 'package:moon/providers/store_provider.dart';
import 'package:tuple/tuple.dart';

enum PortType { output, input }

List<Widget> addPort(
  // PortType kind,
  // List<String> labels,
  // Color color,
  SplayTreeMap<int, Tuple2<String, rid.NodeView>> ports,
  Ref _ref,
  String commandName,
) {
  return ports.values.map((nodeEntry) {
    // find filled ports

    if (nodeEntry.item2.widgetType == rid.NodeViewType.WidgetInput) {
      return InputPort(
        key: ObjectKey(nodeEntry),
        nodeEntry: nodeEntry,
        commandName: commandName,
      );
    } else {
      return OutputPort(
        key: ObjectKey(nodeEntry),
        nodeEntry: nodeEntry,
        commandName: commandName,
      );
    }
  }).toList();
}

class InputPort extends HookConsumerWidget {
  const InputPort({
    Key? key,
    required this.nodeEntry,
    required this.commandName,
  }) : super(key: key);

  final Tuple2<String, rid.NodeView> nodeEntry;
  // final List edges;
  final String commandName;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    // print("rebuilding input port");
    return Tooltip(
        // triggerMode: TooltipTriggerMode.longPress,
        waitDuration: const Duration(milliseconds: 850),
        decoration: BoxDecoration(
          borderRadius: BorderRadius.circular(5),
          border: Border.all(color: Colors.blueGrey.shade700, width: 1),
          color: Colors.lightBlue.shade50,
        ),
        height: 50,
        padding: const EdgeInsets.all(8.0),
        preferBelow: false,
        textStyle: const TextStyle(fontSize: 18, color: Colors.black87),
        message: nodeEntry.item2.tooltip,
        child: Container(
          width: 120,
          height: 50,
          decoration: const BoxDecoration(
            // backgroundBlendMode: BlendMode.multiply,
            color: Colors.transparent,
          ),
          child: Row(
            children: [
              InputDisk(nodeEntry: nodeEntry, key: ObjectKey(nodeEntry.item2)),
              BasicPort(PortType.input, nodeEntry, commandName)
            ],
          ),
        ));
  }
}

class OutputPort extends HookConsumerWidget {
  const OutputPort({
    Key? key,
    required this.nodeEntry,
    required this.commandName,
  }) : super(key: key);

  final Tuple2<String, rid.NodeView> nodeEntry;
  final String commandName;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    // print(nodeEntry.item1);

    // find highlighted ports
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
        // print("get outbound edges");
        edges = mapList.first.value.flowOutboundEdges;

        final dummy = ref
            .read(edgeController)
            .entries
            .where((element) => element.key == "dummy_edge");

        if (dummy.isNotEmpty && dummy.first.value.from == nodeEntry.item1)
          currentlyDragged = true;
        return mapList.first.value.flowOutboundEdges;
      }
      ;
    }));
    // print(edges);

    return Container(
      width: 120,
      height: 50,
      decoration: BoxDecoration(
        color: Colors.transparent,
      ),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.end,
        children: [
          BasicPort(PortType.output, nodeEntry, commandName),
          Container(
            width: 30,
            child: ElevatedButton(
              onHover: (value) {},
              // color:
              //     edges.contains(nodeEntry.key) ? Colors.green : Colors.white,
              onPressed: () {},
              style: ElevatedButton.styleFrom(
                primary: (edges.length > 0 || currentlyDragged == true) &&
                        highlighted == null
                    ? Colors.amber
                    : highlighted != null
                        ? Color.fromARGB(255, 168, 216, 114)
                        : Colors.white,
                fixedSize: const Size(30, 30),
                shape: const CircleBorder(
                  side: BorderSide(
                    color: Colors.black26,
                    style: BorderStyle.solid,
                  ),
                ),
              ),
              child: edges.length > 1
                  ? Text(
                      edges.length.toString(),
                      textAlign: TextAlign.center,
                    )
                  : null,
            ),
          ),
        ],
      ),
    );
  }
}
