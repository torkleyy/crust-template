<script lang="ts">
    import {
        Button, Card, CardBody, CardHeader, CardText, CardTitle,
        Container, TabContent, TabPane
    } from 'sveltestrap';
    import {fetch} from "@tauri-apps/api/http"
    import {onMount} from "svelte";
    import {Author, Book, Configuration, DefaultApi} from "api";

    const fetchWrapper = async function (url, options) {
        console.log({
            url,
            options
        })

        const response = await fetch(url, options)
        console.log({
            response
        })

        return {
            ...response,
            json: function () {
                return response.data
            }
        }
    }

    onMount(() => {
        const api = new DefaultApi(new Configuration({fetchApi: fetchWrapper, basePath: "http://localhost:8000/v1"}))
        console.log({api})
        api.authors({limit: 20, start: 0}).then(ok => {
            console.log({apiResult: ok})
            authors = ok.rows
        }, err => console.trace({err}))

		api.books({limit: 20, start: 0}).then(ok => {
			console.log({apiResult: ok})
			books = ok.rows
		}, err => console.trace({err}))
    })

    let books: Book[] = []
    let authors: Author[] = [];
</script>

<TabContent>
    <TabPane tabId="books" tab="Books" active>
        <Container fluid class="mt-3">
            {#each books as book}

                <Card class="mb-3">
                    <CardHeader>
                        <CardTitle>{book.title}</CardTitle>
                    </CardHeader>
                    <CardBody>
                        <CardText>
                            Author ID: {book.authorId}
                        </CardText>
                        <CardText><small class="text-muted">Last updated yesterday</small></CardText>
                        <Button color="danger">Delete</Button>
                    </CardBody>
                </Card>

            {/each}
        </Container>
    </TabPane>
    <TabPane tabId="authors" tab="Authors">
        <Container fluid class="mt-3">
            {#each authors as author}

                <Card class="mb-3">
                    <CardHeader>
                        <CardTitle>{author.name}</CardTitle>
                    </CardHeader>
                    <CardBody>
                        <CardText>
                            Date of birth: {new Date(author.birthDate).toLocaleDateString()}
                        </CardText>
                        <CardText><small class="text-muted">Last updated yesterday</small></CardText>
                        <Button color="danger">Delete</Button>
                    </CardBody>
                </Card>

            {/each}
        </Container>
    </TabPane>
</TabContent>

<style>
</style>