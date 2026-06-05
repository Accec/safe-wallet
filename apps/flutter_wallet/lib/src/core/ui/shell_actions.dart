import 'package:flutter/material.dart';

void showShellAction(BuildContext context, String action) {
  ScaffoldMessenger.of(context).showSnackBar(
    SnackBar(content: Text('$action will be connected in the next task.')),
  );
}
