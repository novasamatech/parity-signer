package io.parity.signer.ui.theme

import androidx.compose.material.Colors
import androidx.compose.material.MaterialTheme
import androidx.compose.material.TextFieldDefaults
import androidx.compose.ui.graphics.Color

val Colors.Border600: Color
	get() = if (isLight) Color(0xFFB9C3C8) else Color(0xFF7E8D95)
val Colors.Border500: Color
	get() = if (isLight) Color(0xFFACB7BF) else Color(0xFF556068)
val Colors.Border400: Color
	get() = if (isLight) Color(0xFFDCE3E7) else Color(0xFF334048)

val Colors.Bg600: Color
	get() = if (isLight) Color(0xFF3A3A3C) else Color(0xFF3A3A3C)
val Colors.Bg500: Color
	get() = if (isLight) Color(0xFF343434) else Color(0xFF343434)
val Colors.Bg400: Color
	get() = if (isLight) Color(0xFFEDEDED) else Color(0xFF2E2D2F)
val Colors.Bg300: Color
	get() = if (isLight) Color(0xFFF7FAFD) else Color(0xFF1F2021)
val Colors.Bg200: Color
	get() = if (isLight) Color(0xFFFFFFFF) else Color(0xFF1A1A1B)
val Colors.Bg100: Color
	get() = if (isLight) Color(0xFFF3F4F5) else Color(0xFF111111)
val Colors.Bg000: Color
	get() = if (isLight) Color(0xFFFFFFFF) else Color(0xFF000000)
val Colors.BgDanger: Color
	get() = if (isLight) Color(0xFFFFD3D0) else Color(0xFF2F2424)

val Colors.Text600: Color
	get() = if (isLight) Color(0xFF000000) else Color(0xFFFEFEFE)
val Colors.Text500: Color
	get() = if (isLight) Color(0xFF535353) else Color(0xFFD1D1D1)
val Colors.Text400: Color
	get() = if (isLight) Color(0xFF8F8E8E) else Color(0xFFAEAEAE)
val Colors.Text300: Color
	get() = if (isLight) Color(0xFF7B8287) else Color(0xFF7E8D95)
val Colors.Text100: Color
	get() = if (isLight) Color(0xFF020202) else Color(0xFF334048)

val Colors.Action600: Color
	get() = if (isLight) Color(0xFFFFFFFF) else Color(0xFFFFFFFF)
val Colors.Action400: Color
	get() = if (isLight) Color(0xFF4FA5F5) else Color(0xFF3996EC)
val Colors.Action300: Color
	get() = if (isLight) Color(0xFF2980D7) else Color(0xFF2980D7)
val Colors.Action200: Color
	get() = if (isLight) Color(0xFFB9D4EF) else Color(0xFF0F447A)
val Colors.Action100: Color
	get() = if (isLight) Color(0xFFDFEFFF) else Color(0xFF082542)

val Colors.Crypto500: Color
	get() = if (isLight) Color(0xFFB6EBEE) else Color(0xFF257681)
val Colors.Crypto400: Color
	get() = if (isLight) Color(0xFF39929E) else Color(0xFF65A8B1)
val Colors.Crypto200: Color
	get() = if (isLight) Color(0xFF6CA7AF) else Color(0xFF3D686D)
val Colors.Crypto100: Color
	get() = if (isLight) Color(0xFFDCEDEF) else Color(0xFF21373A)

val Colors.SignalDanger: Color
	get() = if (isLight) Color(0xFFFF3B30) else Color(0xFFFF3B30)
val Colors.SignalOn: Color
	get() = if (isLight) Color(0xFF32D74B) else Color(0xFF32D74B)
val Colors.SignalWarning: Color
	get() = if (isLight) Color(0xFFFFD541) else Color(0xFFFFD541)
