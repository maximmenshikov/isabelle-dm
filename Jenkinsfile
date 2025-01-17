pipeline {
  agent {
    dockerfile {
      filename 'Dockerfile_ubuntu_2304'
      dir 'tools/build-env'
    }
  }

  environment {
    FULL_VERSION = sh(script: "./tools/get_version.sh full", returnStdout: true).trim()
    SHORT_VERSION = sh(script: "./tools/get_version.sh", returnStdout: true).trim()
    BRANCH_FOLDER = sh(script: "./tools/get_branch_folder.sh ${BRANCH_NAME}", returnStdout: true).trim()
  }

  stages {
    stage('Download prerequisites') {
      steps {
        dir('ttg') {
          git url: 'https://github.com/maximmenshikov/ttg.git',
              branch: 'main'
        }
      }
    }
    stage('Perform checks') {
      stages {
        stage("Fixes/formatting") {
          steps {
            sh 'env PATH=${HOME}/.cargo/bin:${PATH} cargo fix && git diff --exit-code'
            sh 'env PATH=${HOME}/.cargo/bin:${PATH} cargo fmt && git diff --exit-code'
          }
        }
        stage("Check version in Git tags") {
          when {
            expression {
              BRANCH_NAME == 'main'
            }
          }
          steps {
            sh 'git tag | grep ${SHORT_VERSION}'
          }
        }
        stage("Check version in Cargo") {
          steps {
            sh 'cat Cargo.toml | grep ${SHORT_VERSION}'
          }
        }
      }
    }
    stage('Build for all platforms') {
      parallel {
        stage('Build (Linux)') {
          steps {
            sh 'env PATH=${HOME}/.cargo/bin:${PATH} cargo build --release'
          }
        }
      }
    }

    stage('Prepare bundle') {
      stages {
        stage('Prepare artifacts (branch)') {
          steps {
            sh 'mkdir -p build && (rm -rf build/* || true)'
            /* Create branch-build-linux and doc-branch-build */
            sh './tools/release.sh --out build/isabelle-dm-${BRANCH_FOLDER}-${BUILD_NUMBER}-linux-x86_64.tar.xz'
            /* Copy branch-build-linux to branch-latest-linux */
            sh 'cp build/isabelle-dm-${BRANCH_FOLDER}-${BUILD_NUMBER}-linux-x86_64.tar.xz build/isabelle-dm-${BRANCH_FOLDER}-latest-linux-x86_64.tar.xz'
          }
        }
        stage('Prepare artifacts (versioned)') {
          when {
            expression {
              BRANCH_NAME == 'main'
            }
          }
          steps {
            /* Create versioned artifacts */
            sh 'mkdir -p build/versioned_artifacts'

            /* Copy branch-latest-linux to fullver-linux */
            sh 'cp build/isabelle-dm-${BRANCH_FOLDER}-latest-linux-x86_64.tar.xz build/versioned_artifacts/isabelle-dm-${FULL_VERSION}-linux-x86_64.tar.xz'
          }
        }
      }
    }
    stage('Publish artifacts') {
      parallel {
        stage('Publish artifacts (branch)') {
          steps {
            ftpPublisher alwaysPublishFromMaster: true,
                         continueOnError: false,
                         failOnError: false,
                         masterNodeName: '',
                         paramPublish: null,
                         publishers: [
                          [
                            configName: 'Isabelle Data Model releases',
                            transfers:
                              [[
                                asciiMode: false,
                                cleanRemote: false,
                                excludes: '',
                                flatten: false,
                                makeEmptyDirs: false,
                                noDefaultExcludes: false,
                                patternSeparator: '[, ]+',
                                remoteDirectory: 'branches/${BRANCH_FOLDER}-${BUILD_NUMBER}',
                                remoteDirectorySDF: false,
                                removePrefix: 'build',
                                sourceFiles: 'build/isabelle-dm-*${BRANCH_FOLDER}-${BUILD_NUMBER}*.tar.xz'
                              ]],
                            usePromotionTimestamp: false,
                            useWorkspaceInPromotion: false,
                            verbose: true
                          ]
                        ]
            ftpPublisher alwaysPublishFromMaster: true,
                         continueOnError: false,
                         failOnError: false,
                         masterNodeName: '',
                         paramPublish: null,
                         publishers: [
                          [
                            configName: 'Isabelle Data Model releases',
                            transfers:
                              [[
                                asciiMode: false,
                                cleanRemote: false,
                                excludes: '',
                                flatten: false,
                                makeEmptyDirs: false,
                                noDefaultExcludes: false,
                                patternSeparator: '[, ]+',
                                remoteDirectory: 'branches/${BRANCH_FOLDER}',
                                remoteDirectorySDF: false,
                                removePrefix: 'build',
                                sourceFiles: 'build/isabelle-dm-*${BRANCH_FOLDER}-latest*.tar.xz'
                              ]],
                            usePromotionTimestamp: false,
                            useWorkspaceInPromotion: false,
                            verbose: true
                          ]
                        ]
          }
        }
        stage('Publish artifacts (versioned)') {
          when {
            expression {
              BRANCH_NAME == 'main'
            }
          }
          steps {
            ftpPublisher alwaysPublishFromMaster: true,
                         continueOnError: false,
                         failOnError: false,
                         masterNodeName: '',
                         paramPublish: null,
                         publishers: [
                          [
                            configName: 'Isabelle Data Model releases',
                            transfers:
                              [[
                                asciiMode: false,
                                cleanRemote: false,
                                excludes: '',
                                flatten: false,
                                makeEmptyDirs: false,
                                noDefaultExcludes: false,
                                patternSeparator: '[, ]+',
                                remoteDirectory: "${FULL_VERSION}",
                                remoteDirectorySDF: false,
                                removePrefix: 'build/versioned_artifacts',
                                sourceFiles: 'build/versioned_artifacts/isabelle-dm-*.tar.xz'
                              ]],
                            usePromotionTimestamp: false,
                            useWorkspaceInPromotion: false,
                            verbose: true
                          ]
                        ]
          }
        }
        stage('Archive artifacts for Jenkins') {
          steps {
            archiveArtifacts artifacts: 'build/isabelle-dm-*.tar.xz'
          }
        }
      }
    }
  }
  post {
    success {
      sh './ttg/ttg_send_notification --env --ignore-bad -- "${JOB_NAME}/${BUILD_NUMBER}: PASSED"'
    }
    failure {
      sh './ttg/ttg_send_notification --env --ignore-bad -- "${JOB_NAME}/${BUILD_NUMBER}: FAILED. See details in ${BUILD_URL}"'
    }
  }
}
