package io.parity.signer.screens.scan.bananasplitcreate


object BananaSplit {
	fun getMinShards(totalShards: Int): Int {
		return totalShards/2 + 1
	}
	const val defaultShards = 4
}
