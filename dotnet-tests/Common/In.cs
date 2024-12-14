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

public class ChangeValueMessage : IInMessage
{
    public InMessageType Type { get; set; } = InMessageType.ChangeValue;

    public string? NodeId { get; set; }
    public string? Value { get; set; }
}

public enum InMessageType
{
    Shutdown,
    ChangeValue,
}

class InMessageConverter : TaggedUnionConverter<IInMessage, InMessageType>
{
    protected override string TagName => "type";

    protected override IInMessage? FromEnum(JsonDocument document, JsonSerializerOptions options, InMessageType type)
    {
        return type switch
        {
            InMessageType.Shutdown => document.Deserialize<ShutdownMessage>(options),
            InMessageType.ChangeValue => document.Deserialize<ChangeValueMessage>(options),
            _ => throw new JsonException("Unknown type variant")
        };
    }
}