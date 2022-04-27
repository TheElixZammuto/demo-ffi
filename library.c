#include "library.h"

#include <stdio.h>
#include <d3d11.h>

int test_ffi(ID3D11Device *device){
    ID3D11DeviceContext *context;
    device->lpVtbl->GetImmediateContext(device, &context);
    if (!context) return 1;
    return 0;
}

int c_test(){
    printf("Hello Test!\n");
    //DirectX Initializazion
    D3D_FEATURE_LEVEL featureLevels[] = {
            D3D_FEATURE_LEVEL_11_0,
            D3D_FEATURE_LEVEL_10_1,
            D3D_FEATURE_LEVEL_10_0,
            D3D_FEATURE_LEVEL_9_3,
            D3D_FEATURE_LEVEL_9_2,
            D3D_FEATURE_LEVEL_9_1
    };
    ID3D11Device *dev;
    UINT creationFlags = 0;
    D3D11CreateDevice(NULL, D3D_DRIVER_TYPE_HARDWARE, NULL, creationFlags, featureLevels, 6, D3D11_SDK_VERSION, &dev, NULL,NULL);
    int r = test_ffi(dev);
    printf("Result: %d",r);
}

/*int main(){
    c_test();
}*/
