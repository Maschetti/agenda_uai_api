import fastify from "fastify";
import { PrismaClient } from "../generated/prisma";

const prisma = new PrismaClient();
const app = fastify({
  logger: true
});

app.post("/users", async (request, reply) => {
  try {
    const body = request.body as { email: string; name: string };
		
    const user = await prisma.user.create({
      data: {
        email: body.email,
        name: body.name,
        phone_number: "31999999999",
        token: "fake-token-123",
        password: "123456"
      },
    });

    return reply.status(201).send(user);
  } catch (error) {
    app.log.error(error);
    return reply.status(500).send({ error: "Erro ao criar usuário" });
  }
});

app.get("/users", () => {
  return prisma.user.findMany();
});

app.get('/', async (request, reply) => {
  return { message: 'Hello World' };
});

app.listen({ port: 3333 }).then(() => {
  return console.log("Server is running at 3333");
});
