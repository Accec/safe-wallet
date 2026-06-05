import 'package:flutter/material.dart';

const safeWalletQuickMotionDuration = Duration(milliseconds: 160);
const safeWalletMotionDuration = Duration(milliseconds: 240);
const safeWalletLongMotionDuration = Duration(milliseconds: 340);
const safeWalletPageMotionDuration = Duration(milliseconds: 300);
const safeWalletExitMotionDuration = Duration(milliseconds: 150);

const safeWalletMotionCurve = Curves.easeOutCubic;
const safeWalletEmphasizedMotionCurve = Curves.easeOutQuart;
const safeWalletExitMotionCurve = Curves.easeInCubic;

const safeWalletEntranceOffset = Offset(0, 0.032);
const safeWalletPageOffset = Offset(0, 0.018);
const safeWalletPageExitOffset = Offset(0, -0.006);
const safeWalletSwitcherOffset = Offset(0, 0.02);
const safeWalletListOffset = Offset(0, 0.026);

const safeWalletLayerDelayStep = Duration(milliseconds: 55);
const safeWalletListStaggerStep = Duration(milliseconds: 28);
const safeWalletMaxListStaggerIndex = 8;

Duration safeWalletLayerDelay(int layer) {
  final boundedLayer = layer < 0 ? 0 : layer;
  return Duration(
    milliseconds: boundedLayer * safeWalletLayerDelayStep.inMilliseconds,
  );
}

Duration safeWalletListDelay(int index) {
  final boundedIndex = index < 0
      ? 0
      : index > safeWalletMaxListStaggerIndex
      ? safeWalletMaxListStaggerIndex
      : index;
  return Duration(
    milliseconds: boundedIndex * safeWalletListStaggerStep.inMilliseconds,
  );
}
