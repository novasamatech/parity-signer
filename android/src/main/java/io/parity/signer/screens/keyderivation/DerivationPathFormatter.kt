package io.parity.signer.screens.keyderivation


object DerivationPathFormatter {
	fun getPassword(path: String): String? {
		return path.split("///").lastOrNull()
	}
}


//@Composable
//fun ParagraphStyle() {
//    Text(
//        buildAnnotatedString {
//            withStyle(style = ParagraphStyle(lineHeight = 30.sp)) {
//                withStyle(style = SpanStyle(color = Color.Blue)) {
//                    append("Hello\n")
//                }
//                withStyle(
//                    style = SpanStyle(
//                        fontWeight = FontWeight.Bold,
//                        color = Color.Red
//                    )
//                ) {
//                    append("World\n")
//                }
//                append("Compose")
//            }
//        }
//    )
//}
