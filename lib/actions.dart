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

class ActionBar extends HookConsumerWidget {
  const ActionBar({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return Container();
  }
}
