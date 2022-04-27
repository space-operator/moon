import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

///
final focusRejectProvider = Provider<FocusReject>((ref) {
  return FocusReject(ref.read);
});

class FocusReject {
  final Reader _read;
  List<Rect> rects;
  FocusReject(this._read) : this.rects = [Rect.fromLTRB(0, 0, 0, 0)];

  set all(List<Rect> list) {
    // log.v("setting $list");

    rects = list;
  }
}

final focusRejectController =
    StateNotifierProvider<FocusRejectController, List<Rect>>(
        (ref) => FocusRejectController(ref));

class FocusRejectController extends StateNotifier<List<Rect>> {
  final Ref _ref;

  FocusRejectController(this._ref) : super([]) {
    // _subscribe();
  }
  set(List<Rect> list) {
    // log.v("set $list");
    _ref.read(focusRejectProvider).all = list;
    updateState();
  }

  updateState() {
    // log.v("update focus rejection list");
    // state = _ref.refresh(lastChangesRepoProvider); // refreshes too often
    state = _ref.read(focusRejectProvider).rects;
  }
}
