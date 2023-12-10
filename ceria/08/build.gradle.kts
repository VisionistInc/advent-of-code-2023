plugins {
	application
	id("com.github.johnrengelman.shadow") version "8.1.1"
	kotlin("jvm") version "1.9.21"
}

repositories {
	mavenCentral()
}

application {
	mainClass.set("SolutionKt")
}

tasks {
	clean {
		file("${projectDir}/Solution.jar").delete()
	}

    shadowJar {
        archiveBaseName.set("Solution")
		archiveClassifier.set("")
		destinationDirectory.set(projectDir)
        mergeServiceFiles()
    }
}
