import 'package:flutter/material.dart';

import 'package:flutter/services.dart';

Future<void> copyToClipboard(text, buildContext) async {
  await Clipboard.setData(ClipboardData(text: text));
  ScaffoldMessenger.of(buildContext).showSnackBar(SnackBar(
    content: const Text('Copied to clipboard', textAlign: TextAlign.center),
  ));
  // Scaffold.of(context).showSnackBar(snackbar)
}
