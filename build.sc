import mill._, scalalib._, publish._
import mill.define.Sources
import mill.scalalib.JavaModule
import os.Path
import $ivy.`io.github.otavia-projects::mill-rust_mill$MILL_BIN_PLATFORM:0.2.4`
import io.github.otavia.jni.plugin.RustJniModule

object ProjectInfo {

  def description: String = "Some fast bloom filter implemented by Rust for Python and Java!"

  def organization: String = "io.github.yankun1992"

  def organizationUrl: String = ""

  def projectUrl: String = ""

  def licenses = Seq()

  def author = Seq("Yan Kun <yan_kun@icekredit.com>")

  def version = "0.5.5-SNAPSHOT"

  def buildTool = "mill"

  def buildToolVersion = mill.BuildInfo.millVersion

}

object fastbloomjvm extends RustJniModule with PublishModule {

  override def release: Boolean = true

  override def publishVersion: T[String] = ProjectInfo.version

  override def pomSettings: T[PomSettings] = PomSettings(
    description = ProjectInfo.description,
    organization = ProjectInfo.organization,
    url = "",
    licenses = ProjectInfo.licenses,
    versionControl = VersionControl(),
    developers = Seq(Developer("yan_kun", "Yan Kun", "", Some("icekredit"), Some("")))
  )

  override def artifactName = "fastbloomjvm"

  override def artifactId = "fastbloom"

  override def ivyDeps = Agg(ivy"io.github.otavia-projects:jni-loader:0.2.4")


  override def otherNativeLibraries: Seq[PathRef] = Seq(PathRef(os.pwd / "library"))

  object test extends Tests with TestModule.Junit4 {

  }

}