initSidebarItems({"enum":[["CameraPrefab","Projection prefab"],["Projection","The projection mode of a `Camera`."]],"struct":[["ActiveCamera","Active camera resource, used by the renderer to choose which camera to get the view matrix from. If no active camera is found, the first camera will be used as a fallback."],["ActiveCameraPrefab","Active camera prefab"],["Camera","Camera struct."],["CustomMatrix","Provide a custom matrix implementation for various experimental or custom needs. Note that multiple constraints must be met using this in order to be used within Amethyst. Currently, this matrix must be invertible to be used within the engine."],["Orthographic","An appropriate orthographic projection for the coordinate space used by Amethyst. Because we use vulkan coordinates internally and within the rendering engine, normal nalgebra projection objects (`Orthographic3` are incorrect for our use case."],["Perspective","An appropriate orthographic projection for the coordinate space used by Amethyst. Because we use vulkan coordinates internally and within the rendering engine, normal nalgebra projection objects (`Perspective3`) are incorrect for our use case."]]});