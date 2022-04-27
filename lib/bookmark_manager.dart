import 'dart:math';

import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';

import 'package:hooks_riverpod/hooks_riverpod.dart';

import 'package:moon/providers/bookmark.dart';
import 'package:moon/providers/store_provider.dart';
import 'package:plugin/generated/rid_api.dart' as rid;

class BookmarkManager extends HookConsumerWidget {
  const BookmarkManager({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final bookmarks = ref.watch(bookmarkController);
    final store = ref.read(storeRepoProvider).store;

    final hideBookmarks = useState(true);

    final scrollController = useScrollController();

    return Container(
      width: 270,
      height: hideBookmarks.value || bookmarks.length == 0
          ? 50
          : min(300, 35 + bookmarks.length * 60),
      decoration: BoxDecoration(color: Colors.blueGrey[800]),
      child: Column(
        children: [
          GestureDetector(
            behavior: HitTestBehavior.opaque,
            onTap: () {
              hideBookmarks.value = !hideBookmarks.value;
            },
            child: Container(
                width: 270,
                height: 35,
                color: Colors.blueGrey,
                child: Padding(
                  padding: EdgeInsets.all(8),
                  child: Row(children: [
                    if (bookmarks.length != 0)
                      hideBookmarks.value
                          ? const Icon(Icons.arrow_right)
                          : const Icon(Icons.arrow_drop_down),
                    Text("Bookmarks (${bookmarks.length})",
                        style:
                            TextStyle(color: Colors.amberAccent, fontSize: 14)),
                  ]),
                )),
          ),
          if (!hideBookmarks.value)
            Expanded(
              child: ListView.separated(
                controller: scrollController,
                separatorBuilder: ((context, index) => const Divider(
                      height: 10,
                    )),
                itemBuilder: ((context, index) {
                  final bookmark = bookmarks.entries.elementAt(index);

                  return GestureDetector(
                      onTap: () {
                        store.msgGotoBookmark(bookmark.key,
                            timeout: Duration(minutes: 1));
                      },
                      child: Container(
                        height: 50,
                        decoration: BoxDecoration(color: Colors.black12),
                        child: Dismissible(
                          background: Container(
                            alignment: Alignment.centerRight,
                            color: Colors.red,
                            child: Padding(
                              padding: const EdgeInsets.fromLTRB(0, 0, 10, 0),
                              child: const Icon(Icons.cancel),
                            ),
                          ),
                          direction: DismissDirection.endToStart,
                          onDismissed: (direction) {
                            store.msgDeleteBookmark(bookmark.key,
                                timeout: Duration(minutes: 1));
                            ScaffoldMessenger.of(context).showSnackBar(SnackBar(
                              content: Text('Deleted bookmark',
                                  textAlign: TextAlign.center),
                            ));
                          },
                          key: UniqueKey(),
                          child: Padding(
                            padding: const EdgeInsets.all(8.0),
                            child: Text(bookmark.value.name.toString(),
                                style: TextStyle(color: Colors.white)),
                          ),
                        ),
                      ));
                }),
                itemCount: bookmarks.length,
                // children: commandList,
              ),
            ),
        ],
      ),
    );
  }
}
