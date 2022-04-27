import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:plugin/generated/rid_api.dart' as rid;
import 'package:moon/providers/bookmark.dart';
import 'package:moon/widgets/edge.dart';
import 'package:moon/providers/store_provider.dart';

class CanvasLayout extends HookConsumerWidget {
  const CanvasLayout({
    Key? key,
    required BuildContext this.storedContext,
  }) : super(key: key);

  final BuildContext storedContext;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    // print("rebuilding canvas");
    final provider = ref.watch(viewportController);

    final rid.Camera transform;

    if (provider.isEmpty) {
      transform = ref.read(storeRepoProvider).transform;
    } else {
      transform = provider.first;
    }

    final tf = useTransformationController();

    tf.value = Matrix4.identity()
      ..scale(
        transform.scale.numer.toDouble() / transform.scale.denom.toDouble(),
        transform.scale.numer.toDouble() / transform.scale.denom.toDouble(),
      )
      ..translate(
        transform.x.numer.toDouble() / transform.x.denom.toDouble(),
        transform.y.numer.toDouble() / transform.y.denom.toDouble(),
      );

    return OverflowBox(
      alignment: Alignment.topLeft,
      minWidth: 0.0,
      minHeight: 0.0,
      maxWidth: 6000,
      maxHeight: 4000,
      child: Transform(
        transform: tf.value,
        transformHitTests: true,
        child: Container(
          // width: 5000,
          // height: 5000,
          color: Colors.blueGrey.shade800,
          child: Stack(
            children: [
              GridPaper(
                color: Color.fromARGB(144, 120, 144, 156),
                divisions: 1,
                interval: 100,
                subdivisions: 1,
                child: Container(),
              ),
              ref.read(constDraggedEdgeProvider),
              ref.read(constEdgeProvider),
              // Nodes()
              ref.read(constNodeProvider),
            ],
          ),
        ),
      ),
    );
  }
}

final constNodeProvider = Provider<Nodes>((ref) {
  return const Nodes();
});

class Nodes extends HookConsumerWidget {
  const Nodes({
    Key? key,
  }) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    // print("rebuilding nodes");
    final nodes = ref.watch(widgetTreeController).tree.nodeWidgets;
    return Stack(
      children: nodes,
    );
  }
}

final constEdgeProvider = Provider<Edges>((ref) {
  return const Edges();
});

class Edges extends HookConsumerWidget {
  const Edges({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    // print("rebuilding Edges");

    // Iterable edges = [];
    ref.watch(edgeController.select((value) {
      final edges =
          value.entries.where((element) => element.key != "dummy_edge");
      if (edges.isNotEmpty) {
        return edges.length;
      }
      ;
    }));

    //does not account for newly created edge
    final edges = ref
        .read(edgeController)
        .entries
        .where((element) => element.key != "dummy_edge");
    return Stack(
      children: edges.map((edgeElement) => addEdgeWidget(edgeElement)).toList(),
    );
  }
}

final constDraggedEdgeProvider = Provider<DraggedEdge>((ref) {
  return const DraggedEdge();
});

class DraggedEdge extends HookConsumerWidget {
  const DraggedEdge({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    // print("rebuilding Dragged Edges");

    //does not account for newly created edge
    final edge = ref.watch(edgeController.select((value) {
      final draggedEdge =
          value.entries.where((element) => element.key == "dummy_edge");
      if (draggedEdge.isNotEmpty) return draggedEdge.first;
    }));
    // print(edge);
    // final edges = ref.read(edgeController);
    return edge != null ? addEdgeWidget(edge) : Container();
  }
}

EdgeWidget addEdgeWidget(MapEntry<String, rid.EdgeView> edgeElement) {
  EdgeWidget buildEdge = EdgeWidget(
    key: ObjectKey(edgeElement.key),
    edgePainter: EdgePainter(edgeEntry: edgeElement),
  );
  return buildEdge;
}
