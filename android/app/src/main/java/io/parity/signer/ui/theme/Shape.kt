@file:Suppress("unused")

package io.parity.signer.ui.theme

import androidx.compose.foundation.shape.CornerBasedShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.Shapes
import androidx.compose.ui.unit.dp

val Shapes = Shapes(
        small = RoundedCornerShape(4.dp),
        medium = RoundedCornerShape(6.dp),
        large = RoundedCornerShape(8.dp)
)

val Shapes.modal: CornerBasedShape
	get() = RoundedCornerShape(20.dp)
