# 0.1.2

    - Pass option through a configuration struct for more flexibility
    - Add a -O/--optimization option to the CLI to configure shaderc optimization level
    - Add a -e/--env option to choose the target env
    - Add a -s/-spv option to choose the SPIR-V version generated ( defunct, due to https://github.com/google/shaderc/issues/742 )

# 0.1.3

    - Fix a bug where the output directory was not correct on windows
    - Fix a bug where file were saved has ".spriv" instead of ".spirv"
