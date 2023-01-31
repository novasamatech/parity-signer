package io.parity.signer.domain.storage

import android.annotation.SuppressLint
import android.content.Context
import io.parity.signer.domain.getDbNameFromContext
import java.io.File
import java.io.FileOutputStream


/**
 * All interactions should happen only when user is authorised
 */
class DatabaseAssetsInteractor(private val context: Context,
															 private val seedStorage: SeedStorage) {
	private val dbName: String = context.getDbNameFromContext()

	/**
	 * Wipes all data
	 */
	@SuppressLint("ApplySharedPref")
	fun wipe() {
		deleteDir(File(dbName))
		seedStorage.wipe()
	}

	/**
	 * Util to copy Assets to data dir; only used in onBoard().
	 */
	fun copyAsset(path: String) {
		val contents = context.assets.list("Database$path")
		if (contents == null || contents.isEmpty()) {
			copyFileAsset(path)
		} else {
			File(dbName, path).mkdirs()
			for (entry in contents) copyAsset("$path/$entry")
		}
	}

	/**
	 * Util to copy single Assets file
	 */
	private fun copyFileAsset(path: String) {
		val file = File(dbName, path)
		file.createNewFile()
		val input = context.assets.open("Database$path")
		val output = FileOutputStream(file)
		val buffer = ByteArray(1024)
		var read = input.read(buffer)
		while (read != -1) {
			output.write(buffer, 0, read)
			read = input.read(buffer)
		}
		output.close()
		input.close()
	}

	/**
	 * Util to remove directory
	 */
	private fun deleteDir(fileOrDirectory: File) {
		if (fileOrDirectory.isDirectory) {
			val listFiles = fileOrDirectory.listFiles()
			if (!listFiles.isNullOrEmpty()) {
				for (child in listFiles) deleteDir(child)
			}
		}
		fileOrDirectory.delete()
	}
}
