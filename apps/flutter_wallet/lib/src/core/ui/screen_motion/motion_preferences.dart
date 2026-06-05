import 'package:flutter/material.dart';

bool motionDisabled(BuildContext context) {
  return MediaQuery.maybeOf(context)?.disableAnimations ?? false;
}
