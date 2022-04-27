import 'dart:collection';
import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:plugin/generated/rid_api.dart';
import 'package:moon/utils/logger.dart';
import 'package:moon/providers/bookmark.dart';

import 'package:moon/widget_builder.dart';
import 'package:moon/widgets/block.dart';
import 'package:moon/widgets/edge.dart';
import 'package:tuple/tuple.dart';

/// STORE REPO
///
final storeRepoProvider = Provider<StoreRepo>((ref) {
  return StoreRepo(ref.read);
});

class StoreRepo {
  final Reader _read;

  Store store;

  GraphEntry graph_entry;
  HashMap<String, NodeView> nodes;
  HashMap<String, EdgeView> flow_edges;
  List<String> selected_node_ids;
  List<String> selected_command_ids;
  Selection selection;
  Command command;
  List<WidgetTextCommand> text_commands;
  List<GraphEntry> graph_list;
  List<String> highlighted;
  Camera transform;
  Camera transformScreenshot;
  HashMap<String, BookmarkView> bookmarks;
  DebugData debugData;

  StoreRepo(this._read)
      : this.store = Store.instance,
        this.graph_entry = Store.instance.view.graphEntry,
        this.nodes = Store.instance.view.nodes,
        this.flow_edges = Store.instance.view.flowEdges,
        this.selected_node_ids = Store.instance.view.selectedNodeIds,
        this.selected_command_ids = Store.instance.view.selectedCommandIds,
        this.selection = Store.instance.view.selection,
        this.command = Store.instance.view.command,
        this.text_commands = Store.instance.view.textCommands,
        this.graph_list = Store.instance.view.graphList,
        this.highlighted = Store.instance.view.highlighted,
        this.transform = Store.instance.view.transform,
        this.transformScreenshot = Store.instance.view.transformScreenshot,
        this.bookmarks = Store.instance.view.bookmarks,
        this.debugData = Store.instance.view.uiStateDebug {
    // log.v("init StoreRepo");

    updateAll();
  }

  updateAll() {
    this.store = Store.instance;
    this.graph_entry = Store.instance.view.graphEntry;
    this.nodes = Store.instance.view.nodes;
    this.flow_edges = Store.instance.view.flowEdges;
    this.selected_node_ids = Store.instance.view.selectedNodeIds;
    this.selected_command_ids = Store.instance.view.selectedCommandIds;
    this.selection = Store.instance.view.selection;
    this.command = Store.instance.view.command;
    this.text_commands = Store.instance.view.textCommands;
    this.graph_list = Store.instance.view.graphList;
    this.highlighted = Store.instance.view.highlighted;
    this.transform = Store.instance.view.transform;
    this.bookmarks = Store.instance.view.bookmarks;
    this.transformScreenshot = Store.instance.view.transformScreenshot;
    this.debugData = Store.instance.view.uiStateDebug;
  }

  update_nodes() {
    // log.v("update StoreRepo nodes, number of nodes:");

    // store = Store.instance;
    nodes = Store.instance.view.nodes;
  }

  updateFlowEdges() {
    // store = Store.instance;
    flow_edges = Store.instance.view.flowEdges;
  }

  updateHighlighted() {
    // store = Store.instance;

    highlighted = Store.instance.view.highlighted;
  }

  updateViewport() {
    // store = Store.instance;

    transform = Store.instance.view.transform;
  }

  updateBookmarks() {
    // store = Store.instance;

    bookmarks = Store.instance.view.bookmarks;
  }

  updateTransformScreenshot() {
    // store = Store.instance;

    transformScreenshot = Store.instance.view.transformScreenshot;
  }

  updateSelectedNodeIds() {
    // store = Store.instance;

    selected_node_ids = Store.instance.view.selectedNodeIds;
    selected_command_ids = Store.instance.view.selectedCommandIds;
  }

  updateDebugData() {
    // store = Store.instance;

    debugData = Store.instance.view.uiStateDebug;
  }

  updateGraphEntry() {
    // store = Store.instance;
    graph_entry = Store.instance.view.graphEntry;
  }

  updateGraphList() {
    // store = Store.instance;
    graph_list = Store.instance.view.graphList;
  }
}

final lastChangesRepoProvider = Provider<LastChangesRepo>((ref) {
  return LastChangesRepo(ref.read);
});

class LastChangesRepo {
  final Reader _read;

  HashMap<String, NodeChange> changed_nodes_ids;
  List<String> changed_flow_edges_ids;
  bool is_selected_node_ids_changed;
  bool is_selection_changed;
  bool is_command_changed;
  bool is_text_commands_changed;
  bool is_graph_list_changed;
  bool is_highlighted_changed;
  bool is_transform_changed;
  bool is_transform_screenshot_changed;
  bool is_graph_changed;
  bool is_bookmark_changed;

  LastChangesRepo(this._read)
      : this.changed_nodes_ids = Store.instance.lastViewChanges.changedNodesIds,
        this.changed_flow_edges_ids =
            Store.instance.lastViewChanges.changedFlowEdgesIds,
        this.is_selected_node_ids_changed =
            Store.instance.lastViewChanges.isSelectedNodeIdsChanged,
        this.is_selection_changed =
            Store.instance.lastViewChanges.isSelectionChanged,
        this.is_command_changed =
            Store.instance.lastViewChanges.isCommandChanged,
        this.is_text_commands_changed =
            Store.instance.lastViewChanges.isTextCommandsChanged,
        this.is_graph_list_changed =
            Store.instance.lastViewChanges.isGraphListChanged,
        this.is_highlighted_changed =
            Store.instance.lastViewChanges.isHighlightedChanged,
        this.is_transform_changed =
            Store.instance.lastViewChanges.isTransformChanged,
        this.is_transform_screenshot_changed =
            Store.instance.lastViewChanges.isTransformScreenshotChanged,
        this.is_graph_changed = Store.instance.lastViewChanges.isGraphChanged,
        this.is_bookmark_changed =
            Store.instance.lastViewChanges.isBookmarkChanged {
    // log.v("init LastChangesRepo");
    _subscribe();
  }

  update_all_last_changes() {
    // log.v("updating last changes");
    this.changed_nodes_ids = Store.instance.lastViewChanges.changedNodesIds;
    this.changed_flow_edges_ids =
        Store.instance.lastViewChanges.changedFlowEdgesIds;
    this.is_selected_node_ids_changed =
        Store.instance.lastViewChanges.isSelectedNodeIdsChanged;
    this.is_selection_changed =
        Store.instance.lastViewChanges.isSelectionChanged;
    this.is_command_changed = Store.instance.lastViewChanges.isCommandChanged;
    this.is_text_commands_changed =
        Store.instance.lastViewChanges.isTextCommandsChanged;
    this.is_graph_list_changed =
        Store.instance.lastViewChanges.isGraphListChanged;
    this.is_highlighted_changed =
        Store.instance.lastViewChanges.isHighlightedChanged;
    this.is_transform_changed =
        Store.instance.lastViewChanges.isTransformChanged;
    this.is_transform_screenshot_changed =
        Store.instance.lastViewChanges.isTransformScreenshotChanged;
    this.is_graph_changed = Store.instance.lastViewChanges.isGraphChanged;
    this.is_bookmark_changed = Store.instance.lastViewChanges.isBookmarkChanged;
  }

  update_changed_node_ids() {
    this.changed_nodes_ids = Store.instance.lastViewChanges.changedNodesIds;
  }

  // StreamSubscription<PostedConfirm>? _streams;

  void _subscribe() {
    // log.v("subscribe");
    final _streams = rid.replyChannel.stream;

    _streams.listen((ev) {
      // DEBUG
      // _read(storeRepoProvider).updateDebugData();
      // _read(debugController.notifier).updateState();
      switch (ev.type) {
        case Confirm.RequestRefresh:
          {
            _read(storeRepoProvider)
                .store
                .msgRefresh("refresh", timeout: Duration(minutes: 1));
          }
          break;
        case Confirm.RefreshStatus:
          {
            _read(storeRepoProvider).update_nodes();
            _read(widgetTreeController.notifier).build_tree();
            _read(nodeController.notifier).init();
          }
          break;
        case Confirm.RefreshNode:
          {
            update_changed_node_ids();
            // log.v(changed_nodes_ids);

            _read(storeRepoProvider).updateFlowEdges();
            _read(storeRepoProvider).update_nodes();
            _read(treeNodeController.notifier).updateNodes(changed_nodes_ids);

            //  if node ids are received as String
            // List selectedNodes = jsonDecode(ev.data!);
            // List<String> selectedNodesString =
            //     selectedNodes.map((value) => value.toString()).toList();
            // _read(treeNodeController.notifier)
            //     .updateNodesList(selectedNodesString);

            _read(edgeController.notifier).updateState();
          }
          break;
        case Confirm.RefreshDraggedEdge:
          {
            // print("refresh dragged edge");
            update_all_last_changes();

            _read(storeRepoProvider).updateFlowEdges();

            // set highlighted only on drag edge start
            if (ev.data == "start") {
              _read(storeRepoProvider).update_nodes();
              _read(storeRepoProvider).updateHighlighted();

              _read(highlightedPort.notifier).updateState();
              _read(nodeController.notifier).updateState(changed_nodes_ids);
            }

            _read(edgeController.notifier).updateState();
          }
          break;
        case Confirm.UpdatedDimensions:
          {
            log.v("updated node dimensions");
            update_all_last_changes();

            _read(storeRepoProvider).update_nodes();
            _read(storeRepoProvider).updateFlowEdges();

            _read(widgetTreeController.notifier).build_tree();

            _read(nodeController.notifier).updateState(changed_nodes_ids);
            _read(edgeController.notifier).updateState();

            _read(changesController.notifier).updateState();
          }
          break;
        case Confirm.RefreshUI:
          {
            log.v("refresh UI");
            update_all_last_changes();

            List<String>? added_ids = this
                .changed_nodes_ids
                .entries
                .where((element) => element.value.kind == NodeChangeKind.Added)
                .map((e) => e.key)
                .toList();

            // log.v("added ids $added_ids");

            if (this.changed_nodes_ids.isNotEmpty &&
                !this.changed_nodes_ids.keys.contains("dummy_node") &&
                added_ids.isEmpty) {
              log.v("changed nodes - dummy node");

              _read(storeRepoProvider).update_nodes();
              _read(storeRepoProvider).updateFlowEdges();
              _read(edgeController.notifier).updateState();

              _read(treeNodeController.notifier).updateNodes(changed_nodes_ids);
              // _read(nodeController.notifier).updateState(changed_nodes_ids);

              _read(changesController.notifier).updateState();
            }

            if (this.changed_flow_edges_ids.isNotEmpty &&
                ev.data != "end_edge") {
              log.v("changed flow edges");

              _read(storeRepoProvider).updateFlowEdges();

              // update nodes for edges to includes flow edges?
              _read(storeRepoProvider).updateHighlighted();
              _read(highlightedPort.notifier).updateState();

              _read(edgeController.notifier).updateState();
              // _read(changesController.notifier).updateState();
            }
            if (this.changed_flow_edges_ids.isNotEmpty &&
                ev.data == "end_edge") {
              // print("end flow edges");

              _read(storeRepoProvider).updateFlowEdges();
              _read(storeRepoProvider).update_nodes();
              _read(storeRepoProvider).updateHighlighted();

              _read(highlightedPort.notifier).updateState();
              _read(nodeController.notifier).updateState(changed_nodes_ids);

              _read(edgeController.notifier).updateState();
            }
            if (this.is_transform_changed) {
              log.v("is transform changed");
              _read(storeRepoProvider).updateViewport();

              _read(viewportController.notifier).updateState();
            }
            if (this.is_graph_changed) {
              log.v("is graph changed");
              _read(storeRepoProvider).updateGraphEntry();
              _read(storeRepoProvider).updateGraphList();

              _read(graphController.notifier).updateState();
            }
            if (this.is_bookmark_changed) {
              log.v("is bookmark changed");

              _read(storeRepoProvider).updateBookmarks();
              _read(bookmarkController.notifier).updateState();
            }
            if (this.is_transform_screenshot_changed) {
              log.v("is transform screenshot changed");
              _read(storeRepoProvider).updateTransformScreenshot();
              _read(viewportController.notifier).updateToScreenshot();
              _read(transformScreenshotController.notifier).screenshot();
            }
            if (this.is_selected_node_ids_changed || ev.data == "selection") {
              log.v("selected node id changed ${ev.data}");

              _read(storeRepoProvider).updateSelectedNodeIds();
              _read(selectedNodeIds.notifier).updateState();
            }
          }
          break;
        // case Confirm.RefreshUI:
        // case Confirm.RefreshUI:

        case Confirm.ApplyCommand: //must recreate tree
        case Confirm.CreateNode:
        case Confirm.UnDeployed:
          {
            log.v("apply command, create node or undeployed");
            update_all_last_changes();

            _read(storeRepoProvider).update_nodes();
            _read(storeRepoProvider).updateSelectedNodeIds();

            _read(widgetTreeController.notifier).build_tree();
            _read(nodeController.notifier).updateState(changed_nodes_ids);
            _read(changesController.notifier).updateState();
            _read(selectedNodeIds.notifier).updateState();
          }
          break; //
        case Confirm.RemoveNode:
          {
            // find node, unfocus it
            log.v("removing node");
            update_all_last_changes();

            _read(storeRepoProvider).updateAll();
            _read(widgetTreeController.notifier).build_tree();
            _read(nodeController.notifier).init();
            _read(edgeController.notifier).updateState();
            _read(bookmarkController.notifier).updateState();

            _read(changesController.notifier).updateState();
            _read(graphController.notifier).updateState();
          }
          break;
        case Confirm.Initialized:
          {
            log.v("init");
            update_all_last_changes();

            _read(storeRepoProvider).updateAll();

            _read(widgetTreeController.notifier).build_tree();
            _read(edgeController.notifier).updateState();
            _read(bookmarkController.notifier).updateState();

            _read(changesController.notifier).updateState();
            _read(graphController.notifier).updateState();
          }
          break;
        case Confirm.LoadGraph:
          // case Confirm.DeleteGraph:
          {
            log.v("load graph");

            update_all_last_changes();

            _read(storeRepoProvider).updateAll();

            _read(widgetTreeController.notifier).build_tree();
            _read(nodeController.notifier).init();
            _read(edgeController.notifier).updateState();
            _read(bookmarkController.notifier).updateState();

            _read(changesController.notifier).updateState();
            _read(graphController.notifier).updateState();
          }
          break;
        default:
      }

      // _refresh();
    });
  }
}

final selectedNode = Provider.family<ShapeBorder, bool>((ref, selected) {
  return RoundedRectangleBorder(
      side: selected
          ? BorderSide(color: Colors.amber, width: 2)
          : BorderSide.none,
      borderRadius: BorderRadius.circular(5));
});

final debugController = StateNotifierProvider<DebugController, List<DebugData>>(
    (ref) => DebugController(ref));

class DebugController extends StateNotifier<List<DebugData>> {
  final Ref _ref;

  DebugController(this._ref) : super([]) {
    // updateState();
  }

  updateState() {
    // log.v("update changesController");
    state = [_ref.read(storeRepoProvider).debugData];
  }
}

final changesController =
    StateNotifierProvider<ChangesController, LastChangesRepo>(
        (ref) => ChangesController(ref));

class ChangesController extends StateNotifier<LastChangesRepo> {
  final Ref _ref;

  ChangesController(this._ref) : super(LastChangesRepo(_ref.read)) {
    // _subscribe();
  }

  updateState() {
    // log.v("update changesController");
    state = _ref.read(lastChangesRepoProvider);
  }
}

final selectedNodeIds = StateNotifierProvider<SelectedNodeIds, List<String>>(
    (ref) => SelectedNodeIds(ref));

class SelectedNodeIds extends StateNotifier<List<String>> {
  final Ref _ref;

  SelectedNodeIds(this._ref) : super([]) {
    updateState();
  }

  updateState() {
    // log.v("update selectedNodeIds");
    state = _ref.read(storeRepoProvider).selected_node_ids;
  }
}

final highlightedPort = StateNotifierProvider<HighlightedPort, List<String>>(
    (ref) => HighlightedPort(ref));

class HighlightedPort extends StateNotifier<List<String>> {
  final Ref _ref;

  HighlightedPort(this._ref) : super([]) {
    updateState();
  }

  updateState() {
    // log.v("update selectedNodeIds");
    state = _ref.read(storeRepoProvider).highlighted;
  }
}

final graphController =
    StateNotifierProvider<GraphController, List<GraphEntry>>(
        (ref) => GraphController(ref));

class GraphController extends StateNotifier<List<GraphEntry>> {
  final Ref _ref;

  GraphController(this._ref) : super([]) {
    // _subscribe();
  }

  updateState() {
    // log.v("update changesController");
    state = [_ref.read(storeRepoProvider).graph_entry];
  }
}

final contextProvider = Provider<StoredContext>((ref) {
  return StoredContext(ref.read);
});

class StoredContext {
  StoredContext(this._read);

  final Reader _read;
  BuildContext? context;

  BuildContext update(context) {
    this.context = context;
    return context;
  }
}

final contextController =
    StateNotifierProvider<StoredContextController, List<BuildContext>>(
        (ref) => StoredContextController(ref));

class StoredContextController extends StateNotifier<List<BuildContext>> {
  final Ref _ref;

  StoredContextController(this._ref) : super([]) {
    // _subscribe();
  }

  update(context) {
    state = [_ref.read(contextProvider).update(context)];
  }
}

///
//////
final edgeController =
    StateNotifierProvider<EdgeController, HashMap<String, EdgeView>>(
        (ref) => EdgeController(ref));

class EdgeController extends StateNotifier<HashMap<String, EdgeView>> {
  EdgeController(this._ref) : super(HashMap<String, EdgeView>()) {
    init();
  }

  final Ref _ref;
  init() {
    // log.v("init nodeController state");

    state = _ref.read(storeRepoProvider).flow_edges;
  }

  refresh(edge_ids) {
    state = _ref.read(storeRepoProvider).flow_edges;
  }

  updateState() {
    state = _ref.read(storeRepoProvider).flow_edges;
  }
}

final nodeController =
    StateNotifierProvider<StoreController, HashMap<String, NodeView>>(
        (ref) => StoreController(ref));

class StoreController extends StateNotifier<HashMap<String, NodeView>> {
  final Ref _ref;

  StoreController(this._ref) : super(HashMap<String, NodeView>()) {
    // _subscribe();
    init();
  }

  init() {
    // log.v("init nodeController state");

    state = _ref.read(storeRepoProvider).nodes;
  }

  updateState(HashMap<String, NodeChange> node_change) {
    // log.v("update node controller state ");
    _ref
        .read(treeNodeController.notifier)
        .updateNodes(node_change); // does not account for removed

    state = _ref.read(storeRepoProvider).nodes;
  }
}

///
final treeNodeProvider = Provider.family<TreeNode, Tuple3>(
  (ref, tuple3) => TreeNode(
      node: tuple3.item1, selected: tuple3.item2, children: tuple3.item3),
);

class TreeNode {
  MapEntry<String, NodeView> node;
  final bool selected;
  final List<SuperBlock>? children;

  TreeNode({
    required this.node,
    required this.selected,
    required this.children,
  });
}

///
final treeNodeRepoProvider = Provider<TreeNodeRepo>((ref) {
  return TreeNodeRepo(ref.read);
});

class TreeNodeRepo {
  final List<TreeNode> treeNodes;
  final Reader _read;
  TreeNodeRepo(this._read) : this.treeNodes = [];

  add(node) {
    treeNodes.add(node);
    return treeNodes;
  }

  clear() {
    treeNodes.clear();
    return treeNodes;
  }

  updateNodes(List<String> nodeIds) {
    // log.v("treeNodes $nodeIds");

    final potentialUpdatableNodes = _read(storeRepoProvider)
        .nodes
        .entries
        .where((element) =>
            element.value.widgetType != NodeViewType.WidgetInput &&
            element.value.widgetType != NodeViewType.WidgetOutput &&
            element.value.widgetType != NodeViewType.DummyEdgeHandle)
        .map((e) => e.key);

    nodeIds.forEach((nodeId) {
      if (potentialUpdatableNodes.contains(nodeId)) {
        // log.v("update treeNodeRepo $nodeId");
        // final treeNode = get(nodeId);
        // final index =
        //     treeNodes.indexWhere((element) => element.node.key == nodeId);

        // treeNodes[index].node = _read(storeRepoProvider)
        //     .nodes
        //     .entries
        //     .where((element) => element.key == nodeId)
        //     .first;

        final index =
            treeNodes.indexWhere((element) => element.node.key == nodeId);

        treeNodes[index].node = _read(storeRepoProvider)
            .nodes
            .entries
            .where((element) => element.key == nodeId)
            .first;
      }
    });

    return treeNodes;
  }

  TreeNode get(node_id) {
    log.v(treeNodes.length);
    log.v(_read(treeNodeRepoProvider).treeNodes.length);
    log.v("get TreeNode $node_id");

    return treeNodes
        .where((element) => element.node.key == node_id)
        .toList()[0];
  }
}

///
final treeNodeController =
    StateNotifierProvider<TreeNodeController, HashMap<int, List<TreeNode>>>(
        (ref) => TreeNodeController(ref));

class TreeNodeController extends StateNotifier<HashMap<int, List<TreeNode>>> {
  final Ref _ref;

  TreeNodeController(this._ref) : super(HashMap<int, List<TreeNode>>()) {
    init();
  }
  init() {
    state = HashMap.from({
      DateTime.now().millisecondsSinceEpoch:
          _ref.read(treeNodeRepoProvider).treeNodes
    });
  }

  add(node) {
    // log.v("add to tree repo");
    state = HashMap.from({
      DateTime.now().millisecondsSinceEpoch:
          _ref.read(treeNodeRepoProvider).add(node)
    });
  }

  clear() {
    // log.v("clear node controller state");

    state = HashMap.from({
      DateTime.now().millisecondsSinceEpoch:
          _ref.read(treeNodeRepoProvider).clear()
    });
  }

  updateNodesList(List<String> node_ids) {
    log.v("treeNodeController updateNode in treeRepo");
    _ref.read(treeNodeRepoProvider).updateNodes(node_ids);
    // state = []; // workaround otherwise Riverpod doesn't see any changes
    state = HashMap.from({
      DateTime.now().millisecondsSinceEpoch:
          _ref.read(treeNodeRepoProvider).treeNodes
    });
  }

  updateNodes(HashMap<String, NodeChange> node_ids) {
    log.v("treeNodeController updateNode in treeRepo");
    _ref.read(treeNodeRepoProvider).updateNodes(node_ids.keys.toList());
    // state = []; // workaround otherwise Riverpod doesn't see any changes
    state = HashMap.from({
      DateTime.now().millisecondsSinceEpoch:
          _ref.read(treeNodeRepoProvider).treeNodes
    });
  }
}

/// WIDGET TREE
///
///
class WidgetTreeContent {
  final HashMap<String, EdgeView> visitedEdgeElements;
  final HashMap<String, NodeView> visitedNodeViews;
  final List<SuperBlock> nodeWidgets;
  // final List<EdgeWidget> edgeWidgets;

  WidgetTreeContent({
    required this.nodeWidgets,
    // required this.edgeWidgets,
    required this.visitedEdgeElements,
    required this.visitedNodeViews,
  });
}

final widgetTreeProvider = Provider<WidgetTreeRepo>((ref) {
  return WidgetTreeRepo(ref);
});

class WidgetTreeRepo {
  late WidgetTreeContent tree;
  Ref _ref;

  WidgetTreeRepo(this._ref) {
    log.v("init widget tree");
    //clear
    _ref.read(treeNodeController).clear();
    this.tree = build();
  }

  WidgetTreeContent build() {
    // clear treeNodeRepo
    _ref.read(treeNodeController).clear();

    log.v("building tree");

    final HashMap<String, NodeView> nodes = _ref.read(storeRepoProvider).nodes;
    final HashMap<String, NodeView> vertexNodes = HashMap.fromEntries(nodes
        .entries
        .where((element) => element.value.widgetType.name == "WidgetBlock"));

    final HashMap<String, EdgeView> flowEdges =
        _ref.read(storeRepoProvider).flow_edges;

    return returnWidgetTreeFunction(nodes, vertexNodes, _ref);
  }
}

final widgetTreeController =
    StateNotifierProvider<WidgetTreeController, WidgetTreeRepo>(
        (ref) => WidgetTreeController(ref));

class WidgetTreeController extends StateNotifier<WidgetTreeRepo> {
  final Ref _ref;

  WidgetTreeController(this._ref) : super(WidgetTreeRepo(_ref));

  build_tree() {
    log.v("widgetTreeController build tree");

    // _read(widgetTreeProvider).build();

    state = WidgetTreeRepo(_ref);
  }
}

// final refreshStreamProvider = StreamProvider.autoDispose<Store>(
//   (ref) async* {
//     Store store = Store.instance;

//     final channel = rid.replyChannel.stream.where((event) {
//       // log.v(event);
//       return event.type == Confirm.Refresh;
//     });

//     final stream = channel.listen((event) {
//       store.msgRefresh("refresh");
//       // store = Store.instance;
//     });

//     ref.onDispose(() {
//       stream.cancel();
//     });

//     // await for (final _ in channel) {
//     //   yield store;
//     // }
//   },
// );
