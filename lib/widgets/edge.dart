import 'dart:ui' as ui;

import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:plugin/generated/rid_api.dart' as rid;
import 'package:moon/providers/store_provider.dart';

import '../utils/logger.dart';

/// Single Edge
///
class EdgePainter extends CustomPainter {
  // UniqueKey key = UniqueKey();
  final MapEntry<String, rid.EdgeView> edgeEntry;
  final rid.EdgeView? edgeView;

  /// Contructor
  ///
  EdgePainter({
    required this.edgeEntry,
    this.edgeView,
  });
  rid.EdgeView? edgeViewVersion;

  ///
  @override
  void paint(ui.Canvas canvas, ui.Size size) {
    edgeView == null
        ? edgeViewVersion = edgeEntry.value
        : edgeViewVersion = edgeView;
    final point1 = Offset(this.edgeViewVersion!.fromCoordsX.toDouble(),
        this.edgeViewVersion!.fromCoordsY.toDouble());
    final point2 = Offset(this.edgeViewVersion!.toCoordsX.toDouble(),
        this.edgeViewVersion!.toCoordsY.toDouble());

    final paint = Paint()
      ..color = Colors.yellow.shade700
      // edgeElement.facets["expired"] == true ? Colors.red : Colors.black
      ..style = PaintingStyle.stroke
      ..strokeWidth = 3; //edgeElement.facets["expired"] == true ? 1 : 4;

    canvas.drawLine(
      point1,
      point2,
      paint,
    );
  }

  @override
  bool shouldRepaint(CustomPainter oldDelegate) {
    return true;
  }

  @override
  bool hitTest(Offset position) {
    log.v("Edge hit test $position $this ");
    return super.hitTest(position)!;
  }
}

/// Create widget from Edge
///
class EdgeWidget extends HookConsumerWidget {
  final EdgePainter edgePainter;

  final ObjectKey key;

  const EdgeWidget({required this.edgePainter, required this.key});

  MapEntry<String, rid.EdgeView> get edgeEntry => edgePainter.edgeEntry;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    // print("rebuilding edge widget ${this.edgeEntry.key}");
    final edgeView = ref.watch(edgeController.select((map) {
      // if (map.values.isNotEmpty)
      final mapEntryIter =
          map.entries.where((mapEntry) => mapEntry.key == this.edgeEntry.key);
      if (mapEntryIter.isNotEmpty) return mapEntryIter.first.value;
    }));
    // final edgeEntry = provider.entries
    //     .firstWhere((element) => element.key == this.edgeEntry.key);

    final edgePainter = EdgePainter(edgeEntry: edgeEntry, edgeView: edgeView);

    return Positioned(
      child: CustomPaint(
        key: ObjectKey(edgeEntry),
        painter: edgePainter,
      ),
    );
  }
}
