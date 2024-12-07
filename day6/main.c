#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

/* PART ONE */
typedef struct
{
	int x;
	int y;
} Vector2;

typedef enum
{
	UP,
	RIGHT,
	DOWN,
	LEFT,
} Direction;

const Vector2 DIRS[4] = {
	{-1, 0},
	{0, 1},
	{1, 0},
	{0, -1},
};

void	free_double_pointer(void **ptr)
{
	for (int i = 0; ptr[i]; ++i)
		free(ptr[i]);
	free(ptr);
}

int	count_lines(const char *line)
{
	int count = 0;
	for (int i = 0; line[i]; ++i)
		if (line[i] == '\n') ++count;
	return (count);
}

char **split(const char *line, char delim)
{
	int n_lines = count_lines(line);
	char **lines = malloc((n_lines + 1) * sizeof(*lines));
	if (!lines) return (NULL);
	size_t offset = 0;
	for (int i = 0; i < n_lines; ++i)
	{
		size_t line_len = strchr(line + offset, delim) - (line + offset);
		lines[i] = strndup(line + offset, line_len);
		if (!lines[i])
		{
			free_double_pointer((void **)lines);
			return (NULL);
		}
		offset += line_len + 1;
	}
	lines[n_lines] = NULL;
	return (lines);
}

char **read_lines_from_file(const char *path)
{
	FILE *f = fopen(path, "r");
	if (!f) return (NULL);
	if (fseek(f, 0, SEEK_END) != 0) exit(1);
	long file_size = ftell(f);
	rewind(f);
	char *line = calloc(file_size + 1, sizeof(*line));
	if (!line) exit(1);
	if (fread(line, file_size, sizeof(*line), f) != 1) exit(1);
	char **lines = split(line, '\n');
	free(line);
	fclose(f);
	return (lines);
}

int	count_n_lines(char **lines)
{
	int count = 0;
	for (int i = 0; lines[i]; ++i) ++count;
	return (count);
}

void	reset_lines(char **lines)
{
	for (int i = 0; lines[i]; ++i)
	{
		for (int j = 0; lines[i][j]; ++j)
		{
			if (lines[i][j] == 'X') lines[i][j] = '.';
		}
	}
}

Vector2	get_player_pos(char **lines)
{
	for (int x = 0; lines[x]; ++x)
	{
		for (int y = 0; lines[x][y]; ++y)
		{
			if (lines[x][y] == '^') return ((Vector2){x, y});
		}
	}
	return ((Vector2){-1, -1});
}

Vector2 vector2_add(Vector2 v1, Vector2 v2)
{
	return ((Vector2){v1.x + v2.x, v1.y + v2.y});
}

bool vector2_equals(Vector2 v1, Vector2 v2)
{
	return (v1.x == v2.x && v1.y == v2.y);
}

bool is_in_bounds(Vector2 pos, char **lines)
{
	return (pos.x >= 0 && pos.x < count_n_lines(lines)
			&& pos.y >= 0 && pos.y < (int)strlen(lines[pos.x]));
}

void	start_moving(char **lines, Vector2 pos, Direction dir)
{
	bool is_moving = true;
	while (is_moving)
	{
		lines[pos.x][pos.y] = 'X';
		Vector2 next_pos = vector2_add(pos, DIRS[dir]);
		if (is_in_bounds(next_pos, lines))
		{
			if (lines[next_pos.x][next_pos.y] == '#') dir = (dir + 1) % 4;
			else pos = next_pos;
		}
		else is_moving = false;
	}
}

int	count_xs(char **lines)
{
	int count = 0;
	for (int i = 0; lines[i]; ++i)
		for (int j = 0; lines[i][j]; ++j)
			if (lines[i][j] == 'X') ++count;
	return (count);
}

void	part_one(char **lines)
{
	Vector2 pos = get_player_pos(lines);
	if (vector2_equals(pos, (Vector2){-1, -1})) return;
	start_moving(lines, pos, UP);
	printf("%d\n", count_xs(lines));
	reset_lines(lines);
	lines[pos.x][pos.y] = '^';
}

/* PART TWO */

typedef struct List
{
	Vector2 pos;
	Direction dir;
	struct List *next;
} List;

List	*list_create(Vector2 pos, Direction dir)
{
	List *l = malloc(sizeof(*l));
	if (!l) return (NULL);
	l->pos = pos;
	l->dir = dir;
	l->next = NULL;
	return (l);
}

bool	list_add_node(List **list, List *node)
{
	if (!list || !node) return (false);
	if (!*list) *list = node;
	else
	{
		List *current = *list;
		while (current->next) current = current->next;
		current->next = node;
	}
	return (true);
}

void	list_clear(List *list)
{
	while (list)
	{
		List *next = list->next;
		free(list);
		list = next;
	}
}

bool	list_contains(List *list, Vector2 pos)
{
	while (list)
	{
		if (vector2_equals(list->pos, pos)) return (true);
		list = list->next;
	}
	return (false);
}

bool	list_find(List *list, Vector2 pos, Direction dir)
{
	while (list)
	{
		if (vector2_equals(list->pos, pos) && list->dir == dir) return (true);
		list = list->next;
	}
	return (false);
}

bool	check_infinite_loop(char **lines, Vector2 pos, Direction dir)
{
	List *visited = NULL;
	Vector2 obstacle = vector2_add(pos, DIRS[dir]);
	for (;;)
	{
		Vector2 next_pos = vector2_add(pos, DIRS[dir]);
		if (!is_in_bounds(next_pos, lines)) break;
		if (lines[next_pos.x][next_pos.y] == '#' || vector2_equals(next_pos, obstacle))
		{
			dir = (dir + 1) % 4;
		}
		else
		{
			if (list_find(visited, pos, dir))
			{
				list_clear(visited);
				return (true);
			}
			if (!list_add_node(&visited, list_create(pos, dir))) break;
			pos = next_pos;
		}
	}
	list_clear(visited);
	return (false);
}

int	count_loops(char **lines, Vector2 pos, Direction dir)
{
	int count = 0;
	List *visited = NULL;
	for (;;)
	{
		Vector2 next_pos = vector2_add(pos, DIRS[dir]);
		if (!is_in_bounds(next_pos, lines)) break;
		if (lines[next_pos.x][next_pos.y] == '#') dir = (dir + 1) % 4;
		else
		{
			if (!list_contains(visited, next_pos) && check_infinite_loop(lines, pos, dir))
			{
				++count;
			}
			if (!list_add_node(&visited, list_create(pos, dir))) break;
			pos = next_pos;
		}
	}
	list_clear(visited);
	return (count);
}

void	part_two(char **lines)
{
	Vector2 pos = get_player_pos(lines);
	if (vector2_equals(pos, (Vector2){-1, -1})) return;
	printf("%d\n", count_loops(lines, pos, UP));
}

int	main(int argc, char **argv)
{
	char **lines;
	if (argc > 1) lines = read_lines_from_file(argv[1]);
	else lines = read_lines_from_file("input.txt");
	if (!lines) return (1);
	part_one(lines);
	part_two(lines);
	free_double_pointer((void **)lines);
	return (0);
}
