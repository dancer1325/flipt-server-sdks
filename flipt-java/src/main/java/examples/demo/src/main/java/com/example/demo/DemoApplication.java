package com.example.demo;

import java.util.Collections;
import java.util.List;

import io.flipt.api.FliptClient;
import io.flipt.api.evaluation.models.BooleanEvaluationResponse;
import io.flipt.api.evaluation.models.EvaluationRequest;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
@SpringBootApplication
public class DemoApplication {

	public static void main(String[] args) {
		SpringApplication.run(DemoApplication.class, args);
	}

	public static class Book {
		public long getId() {
			return id;
		}

		private long id;

		public void setId(long id) {
			this.id = id;
		}
	}

	@GetMapping("/")
	public List<Book> findBooks() {
		final FliptClient fliptClient = FliptClient.builder().build();
		final EvaluationRequest booleanEvaluationRequest =
				EvaluationRequest.builder()
						.namespaceKey("default")
						.flagKey("flag_boolean")
						.entityId("entity")
						.build();
		final BooleanEvaluationResponse booleanEvaluationResponse =
				fliptClient.evaluation().evaluateBoolean(booleanEvaluationRequest);

		final Book book = new Book();
		final Book book2 = new Book();
		book2.setId(2);

		final Book bookToReturn = booleanEvaluationResponse.isEnabled() ? book : book2;

		return Collections.singletonList(bookToReturn);
	}
}
