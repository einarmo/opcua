using System.Text.Json;

namespace Common;

public interface IInMessage
{
    InMessageType Type { get; }
}

public class ShutdownMessage : IInMessage
{
    public InMessageType Type { get; set; } = InMessageType.Shutdown;
}

public enum InMessageType
{
    Shutdown,
}

class InMessageConverter : TaggedUnionConverter<IInMessage, InMessageType>
{
    protected override string TagName => "type";

    protected override IInMessage? FromEnum(JsonDocument document, JsonSerializerOptions options, InMessageType type)
    {
        return type switch
        {
            InMessageType.Shutdown => document.Deserialize<ShutdownMessage>(options),
            _ => throw new JsonException("Unknown type variant")
        };
    }
}