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
