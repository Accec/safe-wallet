import 'package:flutter/material.dart';

class BusyIcon extends StatelessWidget {
  const BusyIcon({super.key, this.dimension = 18});

  final double dimension;

  @override
  Widget build(BuildContext context) {
    return SizedBox.square(
      dimension: dimension,
      child: const CircularProgressIndicator(strokeWidth: 2),
    );
  }
}
