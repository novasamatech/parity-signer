package io.parity.signer.ui.theme

import androidx.compose.material.Colors
import androidx.compose.ui.graphics.Color


val Colors.fill30: Color
	get() = if (isLight) Color(0x4D000000) else Color(0x4DFFFFFF)

val Colors.fill24: Color
	get() = if (isLight) Color(0x3D000000) else Color(0x3DFFFFFF)

val Colors.fill18: Color
	get() = if (isLight) Color(0x2E000000) else Color(0x2EFFFFFF)

val Colors.fill12: Color
	get() = if (isLight) Color(0x1F000000) else Color(0x1FFFFFFF)

val Colors.fill6: Color
	get() = if (isLight) Color(0x0F000000) else Color(0x0FFFFFFF)




val Colors.backgroundPrimary: Color
	get() = if (isLight) Color(0xFFFFFFFF) else Color(0xFF1E1E23)

val Colors.backgroundSecondary: Color
	get() = if (isLight) Color(0xFFFFFFFF) else Color(0xFF2C2C30)

val Colors.textSecondary: Color
	get() = if (isLight) Color(0xA8000000) else Color(0xB0FFFFFF)

val Colors.textTertiary: Color
	get() = if (isLight) Color(0x73000000) else Color(0x7AFFFFFF)



val Colors.pink500: Color
	get() = Color(0xFFE6007A)

val Colors.pink300: Color
	get() = Color(0xFFF272B6)

val Colors.red400: Color
	get() = Color(0xFFFD4935)

val Colors.red500: Color
	get() = Color(0xFFFE8D81)
