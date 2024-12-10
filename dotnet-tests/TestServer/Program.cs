using Common;
using Opc.Ua.Configuration;

namespace TestServer;

internal sealed class Program
{
    private static async Task<int> Main()
    {
        var app = new ApplicationInstance
        {
            ConfigSectionName = "TestServer"
        };
        using var source = new CancellationTokenSource();
        TestServer server;
        try
        {
            var cfg = await app.LoadApplicationConfiguration(Path.Join("dotnet-tests", "TestServer.Config.xml"), true);
            await app.CheckApplicationInstanceCertificate(false, 0);
            server = new TestServer();
            await app.Start(server);
        }
        catch (Exception e)
        {
            Comms.Send(new ErrorMessage
            {
                Message = $"Fatal error: {e}"
            });
            return 1;
        }

        Comms.Send(new ReadyMessage());

        while (!source.Token.IsCancellationRequested)
        {
            await foreach (var message in Comms.ListenToInput(source.Token))
            {
                if (message is ShutdownMessage)
                {
                    source.Cancel();
                }
            }
        }


        return 0;
    }
}
