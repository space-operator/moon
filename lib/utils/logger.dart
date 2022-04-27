import 'package:logger/logger.dart';

/// Logger Utility
///
var log = Logger(
  level: Level.info,
  output: ConsoleOutput(),
  printer: PrettyPrinter(
    methodCount: 0,
    printEmojis: false,
    printTime: false,
    noBoxingByDefault: false,
  ), // Use the PrettyPrinter to format and print log
);
