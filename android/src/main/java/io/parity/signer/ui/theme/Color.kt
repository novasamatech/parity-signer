package io.parity.signer.ui.theme

import androidx.compose.material.Colors
import androidx.compose.ui.graphics.Color

/**
 * Our color schema is shared with iOS and not mapped to android [androidx.compose.material.MaterialTheme] so mainly defined here
 */

val Colors.backgroundSystem: Color //to be renamed - used for seed list only now
	get() = if (isLight) Color(0xFFF3F3F2) else Color(0xFF101015)

val Colors.backgroundPrimary: Color // it's in android theme - background
	get() = if (isLight) Color(0xFFFFFFFF) else Color(0xFF101015)

val Colors.backgroundSecondary: Color
	get() = if (isLight) Color(0xFFFFFFFF) else Color(0xFF1E1E23)

val Colors.backgroundTertiary: Color
	get() = if (isLight) Color(0xFFFFFFFF) else Color(0xFF2C2C30)

val Colors.textSecondary: Color
	get() = if (isLight) Color(0xA8000000) else Color(0xB0FFFFFF)

val Colors.textTertiary: Color
	get() = if (isLight) Color(0x73000000) else Color(0x7AFFFFFF)

val Colors.textTertiaryDarkForced: Color
	get() = Color(0x7AFFFFFF)

val Colors.textDisabled: Color
	get() = if (isLight) Color(0x40000000) else Color(0x45FFFFFF)



val Colors.fill30: Color
	get() = if (isLight) Color(0x4D000000) else Color(0x4DFFFFFF)

/**
 * light fill for light theme and dark for dark. Used to made button disabled
 */
val Colors.fill30Inverted: Color
	get() = if (!isLight) Color(0x4D000000) else Color(0x4DFFFFFF)

val Colors.fill24: Color
	get() = if (isLight) Color(0x3D000000) else Color(0x3DFFFFFF)

val Colors.fill18: Color
	get() = if (isLight) Color(0x2E000000) else Color(0x2EFFFFFF)

val Colors.fill12: Color
	get() = if (isLight) Color(0x1F000000) else Color(0x1FFFFFFF)

val Colors.fill6: Color
	get() = if (isLight) Color(0x0F000000) else Color(0x0FFFFFFF)



val Colors.appliedOverlay: Color
	get() = if (isLight) Color(0x7A000000) else Color(0xB3000000)

val Colors.appliedHover: Color
	get() = if (isLight) Color(0xD2000000) else Color(0xD2FFFFFF)

val Colors.appliedStroke: Color
	get() = if (isLight) Color(0x1F000000) else Color(0x1FFFFFFF)

val Colors.appliedSeparator: Color
	get() = if (isLight) Color(0x14000000) else Color(0x14FFFFFF)



val Colors.pink500: Color
	get() = Color(0xFFE6007A)

val Colors.pink300: Color
	get() = Color(0xFFF272B6)

val Colors.red400: Color
	get() = Color(0xFFFD4935)

val Colors.red500: Color
	get() = Color(0xFFFE8D81)

val Colors.snackBarBackground: Color
	get() = Color(0xFF454549)

val Colors.forcedFill30: Color
	get() = Color(0x4D000000)

val Colors.forcedFill40: Color
	get() = Color(0x66000000)

