import os
import torch
import torchvision

# resnet34
model = torchvision.models.resnet34(weights=torchvision.models.ResNet34_Weights.DEFAULT)
model.eval()
torch.save(dict(model.state_dict()), "resnet34.bin")

# resnet50
model = torchvision.models.resnet50(weights=torchvision.models.ResNet50_Weights.DEFAULT)
model.eval()
torch.save(dict(model.state_dict()), "resnet50.bin")

# resnet101
model = torchvision.models.resnet101(weights=torchvision.models.ResNet101_Weights.DEFAULT)
model.eval()
torch.save(dict(model.state_dict()), "resnet101.bin")

# squeezenet1_1
model = torchvision.models.squeezenet1_1(weights=torchvision.models.SqueezeNet1_1_Weights.DEFAULT)
model.eval()
torch.save(dict(model.state_dict()), "squeezenet1_1.bin")

# vgg16
model = torchvision.models.vgg16(weights=torchvision.models.VGG16_Weights.DEFAULT)
model.eval()
torch.save(dict(model.state_dict()), "vgg16.bin")
