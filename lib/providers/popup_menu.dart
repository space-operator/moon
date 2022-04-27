import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:plugin/generated/rid_api.dart';
import 'package:moon/providers/store_provider.dart';

///
/// provide block id [node_id]
final popUpMenuProvider = Provider.family<PopupMenuButton, String>((
  ref,
  node_id,
) {
  final store = ref.read(storeRepoProvider).store;

  return PopupMenuButton(
    color: Colors.blueGrey[900],
    icon: const Icon(Icons.more_horiz),
    shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(5)),
    itemBuilder: (context) {
      return [
        PopupMenuItem<int>(
          onTap: () {},
          value: 0,
          child: const Text(
            "",
            style: const TextStyle(color: Color.fromARGB(255, 153, 175, 185)),
          ),
        ),
        PopupMenuItem<int>(
          onTap: () {
            // print(FocusScope.of(context).focusedChild);
            store.msgRemoveNode(node_id, timeout: Duration(minutes: 1));
          },
          value: 1,
          child: const Text(
            "delete",
            style: const TextStyle(color: Color.fromARGB(255, 153, 175, 185)),
          ),
        ),
      ];
    },
  );
});
